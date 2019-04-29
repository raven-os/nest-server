pub mod content;
pub mod metadata;
pub mod versions;

use libnest::package::VersionData;
use semver::Version;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
struct PackageVersion {
    pub version: Version,
    pub metadata: VersionData,
}
