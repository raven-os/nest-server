use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use std::collections::HashMap;

pub type PackageId = String;
pub type PackageFullName = String;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    name: String,
    category: String,
    version: Version,
    description: String, // not yet ideal type
    tags: String,        // not yet ideal type
    created_at: DateTime<Utc>,
}

impl Metadata {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn tags(&self) -> &str {
        &self.tags
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Manifest {
    metadata: Metadata,
    #[serde(default)]
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl Manifest {
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
