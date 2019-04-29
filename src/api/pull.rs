use std::sync::{Arc, RwLock};

use libnest::package::PackageManifest;
use rocket::State;
use rocket_contrib::json::Json;

use crate::package::NPFManager;

#[get("/api/pull")]
pub fn pull(npf_manager: State<Arc<RwLock<NPFManager>>>) -> Json<Vec<PackageManifest>> {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    Json(npf_manager.manifests().cloned().collect::<Vec<_>>())
}
