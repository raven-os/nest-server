use std::fs::File;
use std::io::Read;

use failure::Error;
use glob;
use rocket_contrib::Json;
use toml;

use manifest::Manifest;
use RAVEN_REPOSITORY_PATH;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, FromForm)]
pub struct ManifestFilter {
    pub category: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

impl ManifestFilter {
    pub fn category(&self) -> &Option<String> {
        &self.category
    }
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn version(&self) -> &Option<String> {
        &self.version
    }
}

#[get("/search")]
pub fn search() -> Result<Json<Vec<Manifest>>, Error> {
    let mut manifests = Vec::new();
    for path in glob::glob(&format!("{}/**/*.toml", *RAVEN_REPOSITORY_PATH))? {
        let path = path?;
        let mut file = File::open(path)?;

        // Allocate a string long enough to hold the entire file
        let mut s = file
            .metadata()
            .map(|m| String::with_capacity(m.len() as usize))
            .unwrap_or_default();

        file.read_to_string(&mut s)?;
        manifests.push(toml::from_str(&s)?);
    }
    Ok(Json(manifests))
}

#[get("/search?<manifest_filter>")]
fn search_filter(manifest_filter: Option<ManifestFilter>) -> Result<Json<Vec<Manifest>>, Error> {
    let mut manifests = Vec::new();
    for path in glob::glob(&format!("{}/**/*.toml", *RAVEN_REPOSITORY_PATH))? {
        let path = path?;
        let mut file = File::open(path)?;

        // Allocate a string long enough to hold the entire file
        let mut s = file
            .metadata()
            .map(|m| String::with_capacity(m.len() as usize))
            .unwrap_or_default();

        file.read_to_string(&mut s)?;
        manifests.push(toml::from_str(&s)?);
    }

    if let Some(filter) = manifest_filter {
        if let Some(name) = filter.name() {
            manifests.retain(|ref x: &Manifest| x.metadata().name().contains(name));
        }
        if let Some(category) = filter.category() {
            manifests.retain(|ref x: &Manifest| x.metadata().category().contains(category));
        }
    }
    Ok(Json(manifests))
}
