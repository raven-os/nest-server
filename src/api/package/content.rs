use std::sync::{Arc, RwLock};

use libnest::package::PackageID;
use rocket::State;
use rocket_contrib::json::JsonValue;

use crate::config::Config;
use crate::package::NPFManager;
use crate::param::{CategoryNameParam, PackageNameParam, VersionParam};

#[get("/api/p/<category>/<name>/<version>/content")]
pub fn content(
    config: State<Arc<Config>>,
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    category: CategoryNameParam,
    name: PackageNameParam,
    version: VersionParam,
) -> Option<JsonValue> {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    let id = PackageID::from(
        config.name().clone(),
        category.clone().into(),
        name.clone().into(),
        version.clone().into(),
    );

    if let Ok(files) = npf_manager.content_of(&id) {
        serde_json::to_value(files).ok().map(Into::into)
    } else {
        None
    }
}
