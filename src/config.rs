use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use failure::{format_err, Error, ResultExt};
use lazy_static::lazy_static;
use libnest::package::RepositoryName;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref PATH_CONFIG: &'static Path = Path::new("./Repository.toml");
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Config {
    name: RepositoryName,
    pretty_name: String,
    package_dir: PathBuf,
    cache_dir: PathBuf,
    links: Vec<Link>,
    #[serde(default)]
    auth_token: String,
}

impl Config {
    #[inline]
    pub fn load() -> Result<Config, Error> {
        Config::load_from(*PATH_CONFIG)
    }

    #[inline]
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let path = path.as_ref();
        let mut file = File::open(path).context(path.display().to_string())?;

        // Allocate a string long enough to hold the entire file
        let mut s = file
            .metadata()
            .map(|m| String::with_capacity(m.len() as usize))
            .unwrap_or_default();

        file.read_to_string(&mut s)
            .context(path.display().to_string())?;

        // All paths are canonicalized now so it's not needed to do it later
        let mut config: Config = toml::from_str(&s).context(path.display().to_string())?;

        fs::create_dir_all(&config.package_dir)?;
        fs::create_dir_all(&config.cache_dir)?;

        config.package_dir = fs::canonicalize(&config.package_dir)
            .context(config.package_dir.display().to_string())?;

        config.cache_dir =
            fs::canonicalize(&config.cache_dir).context(config.cache_dir.display().to_string())?;

        if let Some(value) = env::var_os("RAVEN_NEST_SERVER_AUTH_TOKEN") {
            config.auth_token = value.to_string_lossy().to_string();
        }

        if config.auth_token.is_empty() {
            Err(format_err!("the authentication token is either empty or not present in both environment and configuration file"))?;
        }

        Ok(config)
    }

    pub fn name(&self) -> &RepositoryName {
        &self.name
    }

    pub fn pretty_name(&self) -> &str {
        &self.pretty_name
    }

    pub fn package_dir(&self) -> &Path {
        &self.package_dir
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn links(&self) -> &[Link] {
        &self.links
    }

    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Link {
    name: String,
    url: String,
    #[serde(default)]
    active: bool,
}
