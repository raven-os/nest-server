#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Metadata {
    name: String,
    category: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Manifest {
    metadata: Metadata,
}

impl Manifest {
    pub fn new(name: &str, category: &str) -> Manifest {
        Manifest {
            metadata: Metadata {
                name: name.to_string(),
                category: category.to_string(),
            },
        }
    }
}

// This is here for debugging purposes until we have a nice database
lazy_static! {
    #[derive(Debug)]
    pub static ref MANIFEST_LIST: Vec<Manifest> = vec![
        Manifest::new("dash", "shell"),
        Manifest::new("gcc-base", "sys-lib"),
        Manifest::new("libc", "sys-lib"),
        Manifest::new("libgcc", "sys-lib"),
        Manifest::new("coreutils", "sys-bin"),
        Manifest::new("libacl", "sys-lib"),
        Manifest::new("libattr", "sys-lib"),
        Manifest::new("libpcre", "sys-lib"),
        Manifest::new("libselinux", "sys-lib"),
    ];
}
