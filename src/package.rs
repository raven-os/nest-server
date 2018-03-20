use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::sync::Mutex;

use toml::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    name: String,
    category: String,
}

impl Package {
    pub fn new(name: &str, category: &mut str) -> Package {
        Package {
            name: name.to_string(),
            category: category.to_string(),
        }
    }
}

lazy_static! {
    pub static ref PACKAGE_LIST: Mutex<Vec<Package>> = Mutex::new(vec![]);
}

pub fn load_packages() -> Result<(), Box<Error>> {
    let mut file = File::open("packages.toml")?;

    let mut packages = String::new();
    file.read_to_string(&mut packages)?;

    let toml = packages.parse::<Value>()?;

    for (name, category) in toml.as_table().unwrap().iter() {
        for (_, value) in category.as_table().unwrap().iter() {
            let package = Package::new(
                &name.to_string(),
                &mut value.to_string().trim_matches('"').to_string(),
            );
            PACKAGE_LIST.lock().unwrap().push(package);
        }
    }
    Ok(())
}
