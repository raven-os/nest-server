use std::fs::File;
use std::io::Read;

use chrono::{DateTime, Utc};
use failure::Error;
use glob;
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::request::FromFormValue;
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
    pub sort_by: Option<String>,
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

    pub fn sort_by(&self) -> &Option<String> {
        &self.sort_by
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

#[post("/search", data = "<manifest_filter>")]
pub fn search(
    manifest_filter: Option<Form<ManifestFilter>>,
) -> Result<rocket_contrib::json::Json<Vec<Manifest>>, Error> {
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
        if let Some(sort_by) = filter.sort_by() {
            match sort_by.as_ref() {
                "name" => manifests.sort_by(|a: &Manifest, b: &Manifest| {
                    a.metadata().name().cmp(&b.metadata().name())
                }),
                "category" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().category().cmp(&b.metadata().category())
                    });
                }
                "version" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().version().cmp(&b.metadata().version())
                    });
                }
                "description" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().description().cmp(&b.metadata().description())
                    });
                }
                "tags" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().tags().cmp(&b.metadata().tags())
                    });
                }
                "created_at" => {
                    manifests.sort_by(|a: &Manifest, b: &Manifest| {
                        a.metadata().created_at().cmp(&b.metadata().created_at())
                    });
                }
                _ => (),
            }
        }
        if let Some(order_by) = filter.order_by() {
            match order_by.as_ref() {
                "desc" => manifests.reverse(),
                _ => (),
            }
        }
    }
    Ok(rocket_contrib::json::Json(manifests))
}
