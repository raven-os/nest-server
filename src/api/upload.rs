use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use failure::Error;
use libnest::package::NPFExplorer;
use rocket::http::Status;
use rocket::{Data, State};

use crate::api::auth::AuthToken;
use crate::config::Config;
use crate::package::gen_tmp_filename;

#[post("/api/upload", data = "<data>")]
pub fn upload(data: Data, config: State<Arc<Config>>, _token: AuthToken) -> Status {
    let tmp_path = gen_tmp_filename();

    let _: Result<(), Error> = try {
        // Write data to /tmp/nest-server/
        if let Some(parent) = tmp_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(&tmp_path)?;
        io::copy(&mut data.open(), &mut file)?;

        let npf_explorer = NPFExplorer::open_at(&tmp_path, "/var/tmp/nest-server")?;
        let id = npf_explorer.manifest().id(config.name().clone());

        // Move file to its final destination: `./cache/<category>/<package>/<name>-<version>.nest`.
        let dst_path = PathBuf::from(config.package_dir())
            .join(id.category().as_ref())
            .join(id.name().as_ref())
            .join(format!("{}-{}.nest", id.name(), id.version().to_string()));

        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // We do a copy and not a rename because `tmp_path` and `dst_path` may not share the same mountpoint
        // The file system notifier will finish the job and update the cache
        fs::copy(&tmp_path, &dst_path)?;
    };

    let _ = fs::remove_file(tmp_path);

    Status::NoContent
}
