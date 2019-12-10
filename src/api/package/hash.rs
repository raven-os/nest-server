use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use data_encoding::HEXUPPER;
use rocket::State;
use rocket_contrib::json::JsonValue;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::config::Config;
use crate::param::{CategoryNameParam, PackageNameParam, VersionParam};

#[get("/api/p/<category>/<name>/<version>/hash")]
pub fn hash(
    config: State<Arc<Config>>,
    category: CategoryNameParam,
    name: PackageNameParam,
    version: VersionParam,
) -> Option<JsonValue> {
    let path = PathBuf::from(config.package_dir())
        .join(category.value().as_ref())
        .join(name.value().as_ref())
        .join(format!(
            "{}-{}.nest",
            name.value().as_ref(),
            version.value()
        ));

    File::open(path)
        .and_then(|mut file| {
            let mut sha256 = Sha256::default();
            std::io::copy(&mut file, &mut sha256).map(|_| {
                let hash = sha256.result();
                JsonValue(json!({
                    "sha256": HEXUPPER.encode(hash.as_ref())
                }))
            })
        })
        .ok()
}
