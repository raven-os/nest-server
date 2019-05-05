use std::sync::{Arc, RwLock};

use libnest::package::PackageShortName;
use rocket::http::Status;
use rocket::State;

use crate::package::NPFManager;
use crate::param::{CategoryNameParam, PackageNameParam, VersionParam};

#[get("/api/p/<category>/<name>/<version>")]
pub fn version(
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    category: CategoryNameParam,
    name: PackageNameParam,
    version: VersionParam,
) -> Status {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    let short_name = PackageShortName::from(category.into(), name.into());

    if let Some(manifest) = npf_manager.manifest_of(&short_name) {
        if manifest.versions().contains_key(&version.into()) {
            return Status::NoContent;
        }
    }
    Status::NotFound
}
