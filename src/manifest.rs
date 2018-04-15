#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Metadatas {
    name: String,
    category: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Manifest {
    metadatas: Metadatas,
}

impl Manifest {
    pub fn new(name: &str, category: &str) -> Manifest {
        Manifest {
            metadatas: Metadatas {
                name: name.to_string(),
                category: category.to_string(),
            }
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
