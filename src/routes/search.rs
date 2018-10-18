use std::fs::File;
use std::io::Read;

use chrono::{DateTime, Utc};
use failure::Error;
use glob;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use rocket_contrib::Json;
use serde_derive::{Deserialize, Serialize};
use toml;

use crate::manifest::Manifest;
use crate::RAVEN_REPOSITORY_PATH;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, FromForm)]
pub struct ManifestFilter {
    pub category: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>, // not yet ideal type
    pub tags: Option<String>,        // not yet ideal type
    pub created_at: Option<FormDateTime>,
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

    pub fn created_at(&self) -> &Option<FormDateTime> {
        &self.created_at
    }

    pub fn order_by(&self) -> &Option<String> {
        &self.order_by
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FormDateTime(DateTime<Utc>);

impl<'v> FromFormValue<'v> for FormDateTime {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormDateTime, &'v RawStr> {
        match form_value.parse::<DateTime<Utc>>() {
            Ok(date) => Ok(FormDateTime(date)),
            _ => Err(form_value),
        }
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
        if let Some(description) = filter.description() {
            manifests.retain(|ref x: &Manifest| x.metadata().description().contains(description));
        }
        if let Some(tags) = filter.tags() {
            manifests.retain(|ref x: &Manifest| x.metadata().tags().contains(tags));
        }
        if let Some(order_by) = filter.order_by() {
            match order_by.as_ref() {
                "name_asc" => manifests.sort_by(|a: &Manifest, b: &Manifest| {
                    a.metadata().name().cmp(&b.metadata().name())
                }),
                "name_desc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        b.metadata().name().cmp(&a.metadata().name())
                    });
                }
                "category_asc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().category().cmp(&b.metadata().category())
                    });
                }
                "category_desc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        b.metadata().category().cmp(&a.metadata().category())
                    });
                }
                "description_asc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().description().cmp(&b.metadata().description())
                    });
                }
                "description_desc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        b.metadata().description().cmp(&a.metadata().description())
                    });
                }
                "tags_asc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().tags().cmp(&b.metadata().tags())
                    });
                }
                "tags_desc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        b.metadata().tags().cmp(&a.metadata().tags())
                    });
                }
                "created_at_asc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().created_at().cmp(&b.metadata().created_at())
                    });
                }
                "created_at_desc" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        b.metadata().created_at().cmp(&a.metadata().created_at())
                    });
                }
                _ => (),
            }
        }
    }
    Ok(Json(manifests))
}
