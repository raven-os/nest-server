use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use rocket::http::Status;
use rocket::State;

use crate::api::auth::AuthToken;
use crate::config::Config;
use crate::param::{CategoryNameParam, PackageNameParam, VersionParam};

#[delete("/api/p/<category>/<name>/<version>")]
pub fn delete(
    config: State<Arc<Config>>,
    _token: AuthToken,
    category: CategoryNameParam,
    name: PackageNameParam,
    version: VersionParam,
) -> Status {
    let path = PathBuf::from(config.package_dir())
        .join(category.value().as_ref())
        .join(name.value().as_ref())
        .join(format!(
            "{}-{}.nest",
            name.value().as_ref(),
            version.value()
        ));

    // Remove the NPF.
    // The file system notifier will finish the job and update the cache
    if path.exists() && path.is_file() {
        match fs::remove_file(&path) {
            Ok(_) => Status::NoContent,
            Err(_) => Status::InternalServerError,
        }
    } else {
        Status::NotFound
    }
}
