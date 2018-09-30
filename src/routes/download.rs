use std::path::PathBuf;

use crate::RAVEN_REPOSITORY_PATH;
use rocket::response::NamedFile;

#[get("/download/<path..>")]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn download(path: PathBuf) -> Option<NamedFile> {
    let path = PathBuf::from(&*RAVEN_REPOSITORY_PATH)
        .join(path)
        .join("data")
        .with_extension("tar.gz");
    NamedFile::open(&path).ok()
}
