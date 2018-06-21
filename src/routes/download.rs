use std::path::PathBuf;

use rocket::response::NamedFile;
use RAVEN_REPOSITORY_PATH;

#[get("/download/<path..>")]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn download(path: PathBuf) -> Option<NamedFile> {
    let path = PathBuf::from(&*RAVEN_REPOSITORY_PATH)
        .join(path)
        .join("data")
        .with_extension("tar.gz");
    NamedFile::open(&path).ok()
}
