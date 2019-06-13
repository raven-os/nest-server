pub mod history;
pub mod notify;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read};
use std::iter;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use failure::{format_err, Error, Fail};
use flate2::read::GzDecoder;
use libnest::package::{
    CategoryName, Kind, Manifest, NPFExplorer, PackageFullName, PackageID, PackageManifest,
    PackageName, PackageShortName, VersionData,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use semver::Version;
use serde::{Deserialize, Serialize};
use tar::Archive;

use crate::config::Config;
use crate::package::history::History;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Fail)]
#[fail(display = "{}: invalid NPF path or name", _0)]
pub struct ParseNPFPathError(String);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Fail)]
#[fail(display = "{}: NPF content and path don't match", _0)]
pub struct NPFPackageIDError(String);

/// Result of a content search
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ContentSearchResult {
    path: PathBuf,
    name: PackageFullName,
    all_versions: bool,
}

/// Manager of all NPF.
///
/// This structure transparently caches the PackageManifest and list of files
/// of each package.
pub struct NPFManager {
    config: Arc<Config>,
    manifests: HashMap<PackageShortName, PackageManifest>,
    history: History,
}

impl NPFManager {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            manifests: HashMap::new(),
            history: History::new(),
        }
    }

    pub fn manifests_count(&self) -> usize {
        self.manifests.len()
    }

    pub fn manifests(&self) -> impl Iterator<Item = &PackageManifest> {
        self.manifests.values()
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    /// Find the manifest with the given name.
    ///
    /// The search is cached to speed-up the results.
    pub fn manifest_of(&self, name: &PackageShortName) -> Option<&PackageManifest> {
        self.manifests.get(name)
    }

    /// Find the content of the package with the given name.
    ///
    /// # Note
    ///
    /// If a package doesn't have a content (i.e. it is a virtual package), this function returns `Ok(None)`.
    ///
    /// The search is cached to speed-up the results.
    pub fn content_of(&self, id: &PackageID) -> Result<Option<Vec<PathBuf>>, Error> {
        let cache_entry = NPFCacheEntry::from(&self.config, &id);
        if cache_entry.exists() {
            cache_entry.filesmap()
        } else {
            Err(format_err!("{}: package not found", id))
        }
    }

    /// Find the packages that contain the given file.
    ///
    /// This search has a terrible complexity, it is the current worst-case of this cache algorithm.
    pub fn browse_packages_for_file(
        &self,
        content: &str,
        exact_match: bool,
    ) -> Result<Vec<ContentSearchResult>, Error> {
        let mut results = Vec::new();

        for manifest in self.manifests.values() {
            let mut counters: HashMap<PathBuf, usize> = HashMap::new();

            for version in manifest.versions().keys() {
                let id = PackageID::from_full_name(manifest.full_name(), version.clone());
                let entry = NPFCacheEntry::from(&self.config, &id);

                for result in entry.contains(content, exact_match)? {
                    let counter = counters.entry(result).or_default();
                    *counter += 1;
                }
            }

            let mut tmp_res = counters
                .into_iter()
                .map(|(path, counter)| ContentSearchResult {
                    path,
                    name: manifest.full_name(),
                    all_versions: counter == manifest.versions().len(),
                })
                .collect::<Vec<_>>();

            results.append(&mut tmp_res);
        }
        Ok(results)
    }

    /// Parse the path of an NPF to retrieve the [`PackageID`] it represents.
    ///
    ///  The NPF must have the following relative path and name: `./<category>/<name>/<name>-<version>.nest`.
    fn parse_npf_path(&self, npf: &Path) -> Result<PackageID, Error> {
        let rel_path = npf.strip_prefix(self.config.package_dir())?;
        let invalid_npf = || ParseNPFPathError(npf.to_path_buf().display().to_string());

        let category_name = rel_path
            .parent()
            .and_then(Path::parent)
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .ok_or_else(invalid_npf)?;

        let package_name1 = rel_path
            .parent()
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .ok_or_else(invalid_npf)?;

        // Here's the tricky part.
        //
        // NPF are named `name-version`, which can lead to hard names to parse, like
        // `linux-doc-5.0.1-z`.
        //
        // The trick is to split on the `-` and find the first part with a `.`: that's where
        // the version starts.

        let file_parts = rel_path
            .file_stem()
            .and_then(OsStr::to_str)
            .ok_or_else(invalid_npf)?
            .split("-");

        let mut package_name2 = String::new();
        let mut version = String::new();

        let mut modified = &mut package_name2;

        for part in file_parts {
            if part.contains('.') {
                modified.pop();
                modified = &mut version;
            }

            modified.push_str(part);
            modified.push('-')
        }
        modified.pop();

        // Ensure that `package_name1` is equal to `package_name2`
        if package_name1 != package_name2 {
            Err(invalid_npf())?;
        }

        let category = CategoryName::parse(category_name)?;
        let package = PackageName::parse(package_name1)?;
        let version = Version::parse(&version)?;

        Ok(PackageID::from(
            self.config.name().clone(),
            category,
            package,
            version,
        ))
    }

    /// Add a manifest in the cache and history
    fn add_manifest(&mut self, short_name: PackageShortName, manifest: &Manifest) {
        // Extract the manifest from the current HashMap
        let mut pkg_manifest = self
            .manifests
            .remove(&short_name)
            .unwrap_or(PackageManifest::new(
                manifest.name().clone(),
                manifest.category().clone(),
                self.config.name().clone(),
                manifest.metadata().clone(),
            ));

        // Replace metadata iff this is the most up-to-date version according to their wrap_date
        let mut versions_data = pkg_manifest.versions().values().collect::<Vec<_>>();
        versions_data.sort_unstable_by(|a, b| b.wrap_date().cmp(a.wrap_date()));

        if let Some(last_version_data) = versions_data.get(0) {
            if last_version_data.wrap_date() < manifest.wrap_date() {
                *pkg_manifest.metadata_mut() = manifest.metadata().clone();
            }
        }

        // Add the new version
        pkg_manifest.versions_mut().insert(
            manifest.version().clone(),
            VersionData::from(
                manifest.slot().clone(),
                manifest.kind().clone(),
                manifest.wrap_date().clone(),
                manifest.dependencies().clone(),
            ),
        );

        // Update the HashMap entry
        self.manifests.insert(short_name, pkg_manifest.clone());

        // Insert the update in the history
        self.history.add_manifest(pkg_manifest);
    }

    /// Flush (force update) the cache of an existing manifest
    fn add(&mut self, npf: &Path) -> Result<(), Error> {
        let id = self.parse_npf_path(npf)?;
        let short_name: PackageShortName = id.clone().into();

        let mut cache_entry = NPFCacheEntry::from(&self.config, &id);

        // Any error past that point result in a cache purge to ensure integrity
        // and avoid half-empty, half-full cache entries.
        let r: Result<(), Error> = try {
            cache_entry.fill_with(npf)?;

            let manifest = cache_entry.manifest()?;

            // Ensure the manifest contained within the NPF actually matches its path
            if manifest.name() != id.name()
                || manifest.category() != id.category()
                || manifest.version() != id.version()
            {
                Err(NPFPackageIDError(npf.to_path_buf().display().to_string()))?;
            }

            self.add_manifest(short_name, &manifest);

            println!("[CACHE] Cache updated for {}", id);
        };

        if r.is_err() && cache_entry.exists() {
            cache_entry.purge()?;
        };
        r
    }

    /// Flush (remove) the cache of a removed manifest
    fn remove(&mut self, npf: &Path) -> Result<(), Error> {
        let id = self.parse_npf_path(npf)?;
        let short_name: PackageShortName = id.clone().into();

        let mut cache_entry = NPFCacheEntry::from(&self.config, &id);

        if cache_entry.exists() {
            cache_entry.purge()?;
        }

        if let Some(manifest) = self.manifests.get_mut(&short_name) {
            manifest.versions_mut().remove(id.version());
            if manifest.versions().len() == 0 {
                self.manifests.remove(&short_name);
            }
        }

        println!("[CACHE] Cache removed for {}", id);

        Ok(())
    }

    /// Flush the cache entry of an arbitrary NPF.
    ///
    /// If the file doesn't exist, the entry is removed from the cache.
    /// Non NPF files are ignored for simplicity.
    pub fn flush<P: AsRef<Path>>(&mut self, npf: P) -> Result<(), Error> {
        let npf = npf.as_ref();

        // Ignore non-NPF files
        if npf.extension().and_then(OsStr::to_str) == Some("nest") {
            let r: Result<_, Error> = {
                if npf.exists() {
                    self.add(npf)
                } else {
                    self.remove(npf)
                }
            };

            if let Err(e) = &r {
                eprintln!(
                    "[CACHE] Failed to update cache for \"{}\": {}",
                    npf.display(),
                    e
                );
            }

            r
        } else {
            Ok(())
        }
    }

    /// Ensure the cache of all manifests isn't dirty, flush it otherwise.
    pub fn resync(&mut self) -> Result<(), Error> {
        self.manifests.clear();
        self.history.clear();

        for npf_path in glob::glob(&format!(
            "{}/*/*/*.nest",
            self.config.package_dir().display()
        ))? {
            let _: Result<(), Error> = try {
                let npf_path = npf_path?;
                let id = self.parse_npf_path(&npf_path)?;

                let cache_entry = NPFCacheEntry::from(&self.config, &id);
                if !cache_entry.exists() || cache_entry.is_dirty()? {
                    println!("[CACHE] Cache dirty for {}, flushing.", id);
                    self.flush(npf_path)?;
                } else {
                    println!("[CACHE] Cache is up to date for {}.", id);
                    self.add_manifest(id.into(), &cache_entry.manifest()?);
                }
            };
        }

        Ok(())
    }
}

pub struct NPFCacheEntry {
    cache_path: PathBuf,
    npf_path: PathBuf,
    manifest_path: PathBuf,
    filesmap_path: PathBuf,
}

impl NPFCacheEntry {
    pub fn from(config: &Config, id: &PackageID) -> Self {
        let cache_path = config
            .cache_dir()
            .to_path_buf()
            .join(id.category().as_ref())
            .join(id.name().as_ref())
            .join(&id.version().to_string());

        let npf_path = PathBuf::from(config.package_dir())
            .join(id.category().as_ref())
            .join(id.name().as_ref())
            .join(format!("{}-{}.nest", id.name(), id.version().to_string()));

        Self {
            npf_path,
            manifest_path: cache_path.join("manifest.toml"),
            filesmap_path: cache_path.join("files.map"),
            cache_path,
        }
    }

    /// Test if the cache entry has been filled previously;
    pub fn exists(&self) -> bool {
        self.cache_path.exists()
    }

    /// Purge the cache entry
    pub fn purge(&mut self) -> io::Result<()> {
        fs::remove_dir_all(&self.cache_path)
    }

    /// Test if the cache entry is dirty
    pub fn is_dirty(&self) -> Result<bool, Error> {
        if self.npf_path.exists() && self.manifest_path.exists() {
            let npf_modified = fs::metadata(&self.npf_path)?.modified()?;
            let manifest_modified = fs::metadata(&self.manifest_path)?.modified()?;
            if self.filesmap_path.exists() {
                let filesmap_modified = fs::metadata(&self.filesmap_path)?.modified()?;
                Ok(manifest_modified < npf_modified && filesmap_modified < npf_modified)
            } else {
                Ok(manifest_modified < npf_modified)
            }
        } else {
            Ok(false)
        }
    }

    /// Fill the cache entry of the given NPF
    ///
    /// This function explores the NPF by extracting it in a temporary folder.
    pub fn fill_with<P: AsRef<Path>>(&mut self, npf_path: P) -> Result<(), Error> {
        let npf_explorer = NPFExplorer::open_at(npf_path.as_ref(), "/var/tmp/nest-server")?;
        let manifest = npf_explorer.manifest();

        // If this is a reupload of an already existing package, remove the previous content
        if self.exists() {
            self.purge()?;
        }

        let res: Result<_, Error> = try {
            let mut files = Vec::new();

            // Find all the files within `data.tar.gz` and write their path in `tmp_filesmap_path`.
            if manifest.kind() == Kind::Effective {
                let data_file = npf_explorer.open_data()?.ok_or_else(|| {
                    format_err!("no data found even though the package is effective")
                })?;
                let mut data = Archive::new(GzDecoder::new(data_file.file()));

                for entry in data.entries()? {
                    let entry = entry?;
                    let entry_path = entry.path()?;

                    // Quick beautifier without altering the meaning of the path
                    // before storing and printing it.
                    let mut pretty_path = PathBuf::from("/");

                    for component in entry_path.components() {
                        match component {
                            Component::CurDir => (),
                            _ => pretty_path.push(component),
                        }
                    }
                    files.push(pretty_path.display().to_string());
                }
            }

            // Copy all new files to their destination
            fs::create_dir_all(&self.cache_path)?;

            let mut new_manifest = File::create(&self.manifest_path)?;
            io::copy(npf_explorer.open_manifest()?.file_mut(), &mut new_manifest)?;

            let filesmap = File::create(&self.filesmap_path)?;
            serde_json::to_writer(filesmap, &files)?;
        };

        // Purge on error
        if res.is_err() && self.exists() {
            self.purge()?;
        }

        res
    }

    /// Return the manifest of the package.
    pub fn manifest(&self) -> Result<Manifest, Error> {
        let mut file = File::open(&self.manifest_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(toml::from_str(&content)?)
    }

    /// Return, if it exists, a list of all the content present in the package.
    pub fn filesmap(&self) -> Result<Option<Vec<PathBuf>>, Error> {
        if self.filesmap_path.exists() {
            let file = File::open(&self.filesmap_path)?;
            Ok(Some(serde_json::from_reader(file)?))
        } else {
            Ok(None)
        }
    }

    /// Test if the package contains the given file, returning all the paths that matches the given query
    pub fn contains(&self, query: &str, exact_match: bool) -> Result<Vec<PathBuf>, Error> {
        let mut res = Vec::new();

        if let Some(filesmap) = self.filesmap()? {
            let filesmap = filesmap
                .iter()
                .filter(|filesmap| filesmap.file_name().is_some())
                .map(|filesmap| (filesmap.file_name().unwrap().to_str(), filesmap))
                .filter(|(filename, _)| filename.is_some())
                .map(|(filename, path)| (filename.unwrap(), path));

            if exact_match {
                for (filename, path) in filesmap {
                    if filename == query {
                        res.push(path.clone());
                    }
                }
            } else {
                for (filename, path) in filesmap {
                    if filename.contains(query) {
                        res.push(path.clone());
                    }
                }
            }
        }

        Ok(res)
    }
}

/// Generate a valid path with a random component
pub fn gen_tmp_filename() -> PathBuf {
    let mut rng = thread_rng();
    let name: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(10)
        .collect();

    Path::new("/var/tmp/nest-server").join(&format!("nest_{}", name))
}
