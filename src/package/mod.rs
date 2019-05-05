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
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use libnest::package::{
    CategoryName, Kind, Manifest, PackageFullName, PackageID, PackageManifest, PackageName,
    PackageShortName, VersionData,
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

        let mut file_parts = rel_path
            .file_stem()
            .and_then(OsStr::to_str)
            .ok_or_else(invalid_npf)?
            .split("-");

        // Split the filename
        let package_name2 = file_parts.next().ok_or_else(invalid_npf)?;
        let version = file_parts.next().ok_or_else(invalid_npf)?;

        // Ensure there are no more parts after the version in the file name.
        // And that `package_name1` is equal to `package_name2`
        if file_parts.next().is_some() || package_name1 != package_name2 {
            Err(invalid_npf())?;
        }

        let category = CategoryName::parse(category_name)?;
        let package = PackageName::parse(package_name1)?;
        let version = Version::parse(version)?;

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
                let filesmap_modified = fs::metadata(&self.manifest_path)?.modified()?;
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
    pub fn fill_with<P: AsRef<Path>>(&mut self, npf: P) -> Result<(), Error> {
        let npf_file = File::open(&npf)?;
        let mut archive = Archive::new(npf_file);
        let tmp_extract_path = gen_tmp_filename();

        fs::create_dir(&tmp_extract_path)?;

        let res: Result<_, Error> = try {
            archive.unpack(&tmp_extract_path)?;

            // Bunch of files to analyse/generate
            let tmp_manifest_path = PathBuf::from(&tmp_extract_path).join("manifest.toml");
            let tmp_data_path = PathBuf::from(&tmp_extract_path).join("data.tar.gz");
            let tmp_filesmap_path = PathBuf::from(&tmp_extract_path).join("files.map");

            // Open the Manifest to retrieve the package's metadata.
            let manifest = open_manifest(&tmp_manifest_path)?;

            // Find all the files within `data.tar.gz` and write their path in `tmp_filesmap_path`.
            if manifest.kind() == Kind::Effective {
                let mut files = Vec::new();
                let data_file = File::open(tmp_data_path)?;
                let mut data = Archive::new(GzDecoder::new(data_file));

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

                let filesmap = File::create(&tmp_filesmap_path)?;
                serde_json::to_writer(filesmap, &files)?;
            }

            // If this is a reupload of an already existing package, remove the previous content
            if self.exists() {
                self.purge()?;
            }

            // Copy all new files to their destination
            // We do a copy and not a move because they may not share the same mountpoint

            fs::create_dir_all(&self.cache_path)?;
            fs::copy(&tmp_manifest_path, &self.manifest_path)?;
            fs::copy(&tmp_filesmap_path, &self.filesmap_path)?;
        };
        fs::remove_dir_all(&tmp_extract_path)?;
        res
    }

    /// Return the manifest of the package.
    pub fn manifest(&self) -> Result<Manifest, Error> {
        open_manifest(&self.manifest_path)
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
            let matcher = {
                if exact_match {
                    RegexMatcher::new_line_matcher(&format!("^{}$", regex::escape(query)))?
                } else {
                    RegexMatcher::new_line_matcher(&format!("^.*{}.*$", regex::escape(query)))?
                }
            };

            for file in filesmap {
                Searcher::new().search_slice(
                    &matcher,
                    file.to_string_lossy().as_bytes(),
                    UTF8(|_, _| {
                        res.push(file.clone());
                        Ok(true)
                    }),
                )?;
            }
        }

        Ok(res)
    }
}

fn open_manifest<P: AsRef<Path>>(path: P) -> Result<Manifest, Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(toml::from_str(&content)?)
}

/// Generate a valid path with a random component
pub fn gen_tmp_filename() -> PathBuf {
    let mut rng = thread_rng();
    let name: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(10)
        .collect();

    PathBuf::from("/tmp").join(&format!("nest_{}", name))
}

/// Remove all files or directories generated with [`gen_tmp_filename`].
pub fn clean_tmp_files() -> Result<(), Error> {
    for path in glob::glob("/tmp/nest_*")? {
        let path = path?;
        if let Ok(md) = fs::metadata(&path) {
            if md.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}

/// Opens a NPF and retrieve the [`PackageID`] of the package.
pub fn get_package_id_from_npf<P: AsRef<Path>>(
    config: &Config,
    npf: P,
) -> Result<PackageID, Error> {
    let npf_file = File::open(npf)?;
    let mut archive = Archive::new(npf_file);
    let tmp_extract_path = gen_tmp_filename();

    fs::create_dir(&tmp_extract_path)?;

    let res: Result<_, Error> = try {
        archive.unpack(&tmp_extract_path)?;

        // Open the Manifest to retrieve the package's metadata.
        let tmp_manifest_path = PathBuf::from(&tmp_extract_path).join("manifest.toml");
        let manifest = open_manifest(&tmp_manifest_path)?;

        manifest.id(config.name().clone())
    };
    fs::remove_dir_all(&tmp_extract_path)?;
    res
}
