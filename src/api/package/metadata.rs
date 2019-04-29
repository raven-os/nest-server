use std::sync::{Arc, RwLock};

use libnest::package::PackageShortName;
use rocket::State;
use rocket_contrib::json::JsonValue;

use crate::package::NPFManager;
use crate::param::{CategoryNameParam, PackageNameParam};

#[get("/api/p/<category>/<name>")]
pub fn metadata(
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    category: CategoryNameParam,
    name: PackageNameParam,
) -> Option<JsonValue> {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    let short_name = PackageShortName::from(category.into(), name.into());

    if let Some(manifest) = npf_manager.manifest_of(&short_name) {
        serde_json::to_value(manifest).ok().map(Into::into)
    } else {
        None
    }
}
