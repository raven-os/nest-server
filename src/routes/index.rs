use std::path::{Path, PathBuf};

static ROOT_STATIC_FILES: &'static str = "./static/";

use rocket::response::NamedFile;

#[get("/<files..>", rank = 1)]
pub fn static_files(files: PathBuf) -> Option<NamedFile> {
    let path = Path::new(ROOT_STATIC_FILES).join(files);
    if path.exists() {
        NamedFile::open(path).ok()
    } else {
        NamedFile::open(Path::new(ROOT_STATIC_FILES).join("index.html")).ok()
    }
}

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join("index.html")).ok()
}
