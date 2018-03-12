#[derive(Serialize, Debug, Clone)]
pub struct Package {
    name: String,
    category: String,
}

impl Package {
    pub fn new(name: &str, category: &str) -> Package {
        Package {
            name: name.to_string(),
            category: category.to_string(),
        }
    }
}

// This is here for debugging purposes until we have a nice database
lazy_static! {
    pub static ref PACKAGE_LIST: Vec<Package> = vec![
        Package::new("dash", "shell"),
        Package::new("gcc-base", "sys-lib"),
        Package::new("libc", "sys-lib"),
        Package::new("libgcc", "sys-lib"),
    ];
}
