use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;

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
    pub description: Option<String>,
    pub tags: Option<String>,
    pub created_at: Option<String>,
    pub order_by: Option<String>,
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
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn tags(&self) -> &Option<String> {
        &self.tags
    }
    pub fn created_at(&self) -> &Option<String> {
        &self.created_at
    }
    pub fn order_by(&self) -> &Option<String> {
        &self.order_by
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

    let sorter : fn(&Manifest, &Manifest) -> Ordering;
    sorter = |a, b| b.metadata().created_at().cmp(&a.metadata().created_at()); // Default sort by create_at
    manifests.sort_by(sorter);

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

    let mut sorter : fn(&Manifest, &Manifest) -> Ordering;
    sorter = |a, b| b.metadata().created_at().cmp(&a.metadata().created_at()); // Default sort by create_at

    if let Some(filter) = manifest_filter {
        if let Some(name) = filter.name() {
            manifests.retain(|ref x: &Manifest| x.metadata().name().contains(name));
        }
        if let Some(category) = filter.category() {
            manifests.retain(|ref x: &Manifest| x.metadata().category().contains(category));
        }
        if let Some(description) = filter.description() {
            manifests.retain(|ref x: &Manifest| x.metadata().description().contains(description));
        }
        if let Some(tags) = filter.tags() {
            manifests.retain(|ref x: &Manifest| x.metadata().tags().contains(tags));
        }
        if let Some(created_at) = filter.created_at() {
            manifests.retain(|ref x: &Manifest| x.metadata().created_at().contains(created_at));
        }
        if let Some(order_by) = filter.order_by() {
          match order_by.as_ref() {
            "name_asc" => sorter = |a, b| a.metadata().name().cmp(&b.metadata().name()),
            "name_desc" => sorter = |a, b| b.metadata().name().cmp(&a.metadata().name()),
            "category_asc" => sorter = |a, b| a.metadata().category().cmp(&b.metadata().category()),
            "category_desc" => sorter = |a, b| b.metadata().category().cmp(&a.metadata().category()),
            "description_asc" => sorter = |a, b| a.metadata().description().cmp(&b.metadata().description()),
            "description_desc" => sorter = |a, b| b.metadata().description().cmp(&a.metadata().description()),
            "tags_asc" => sorter = |a, b| a.metadata().tags().cmp(&b.metadata().tags()),
            "tags_desc" => sorter = |a, b| b.metadata().tags().cmp(&a.metadata().tags()),
            "created_at_asc" => sorter = |a, b| a.metadata().created_at().cmp(&b.metadata().created_at()),
            "created_at_desc" => sorter = |a, b| b.metadata().created_at().cmp(&a.metadata().created_at()),
            _ => ()
          }
        }
    }
    manifests.sort_by(sorter);
    Ok(Json(manifests))
}
