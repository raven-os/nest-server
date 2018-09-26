use semver::{Version, VersionReq};
use std::collections::HashMap;

pub type PackageId = String;
pub type PackageFullName = String;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    name: String,
    category: String,
    version: Version,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Manifest {
    metadata: Metadata,
    #[serde(default)]
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl Metadata {
    pub fn name(&self) -> &str { &self.name }
    pub fn category(&self) -> &str { &self.category }
    pub fn version(&self) -> &Version { &self.version }
}

impl Manifest {
    pub fn metadata(&self) -> &Metadata { &self.metadata }
}

/*

impl Manifest {
    pub fn new(name: &str, category: &str, version: &str) -> Manifest {
        Manifest {
            metadata: Metadata {
                name: name.to_string(),
                category: category.to_string(),
                version: Version::parse(version).unwrap(),
            },
            dependencies: HashMap::new(),
        }
    }
}
// This is here for debugging purposes until we have a nice database
lazy_static! {
    #[derive(Debug)]
    pub static ref MANIFEST_LIST: Vec<Manifest> = vec![
        Manifest::new("libc",           "sys-lib",       "1.0.0"),
        Manifest::new("libc",           "sys-lib",       "1.0.1"),
        Manifest::new("libc",           "sys-lib",       "1.2.0"),
        Manifest::new("libc-dev",       "sys-lib",       "1.2.0")
            .add_dependencie("stable::sys-lib/libc#=1.2.0"),
        Manifest::new("libgcc",         "sys-lib",       "1.0.0"),
        Manifest::new("gcc-base",       "sys-lib",       "1.0.0"),
        Manifest::new("dash",           "shell",         "1.0.0")
            .add_dependencie("stable::sys-lib/libc#>=1.0.0")
            .add_dependencie("stable::sys-lib/libgcc#>=1.0.0"),
        Manifest::new("gcc",           "sys-dev",         "1.0.0")
            .add_dependencie("stable::sys-lib/libc-dev#>=1.0.0")
            .add_dependencie("stable::sys-lib/libgcc#>=1.0.0")
            .add_dependencie("stable::sys-lib/libc#>=1.0.0")
/*
        Manifest::new("coreutils",      "sys-bin",       Version::parse("1.0.0").unwrap()),
        Manifest::new("libacl",         "sys-lib",       Version::parse("1.0.0").unwrap()),
        Manifest::new("libattr",        "sys-lib",       Version::parse("1.0.0").unwrap()),
        Manifest::new("libpcre",        "sys-lib",       Version::parse("1.0.0").unwrap()),
        Manifest::new("libselinux",     "sys-lib",       Version::parse("1.0.0").unwrap()),
*/
    ];
}
*/
