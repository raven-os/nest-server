use std::path::PathBuf;

use crate::filename::FileName;
use crate::RAVEN_REPOSITORY_PATH;

use rocket::response::NamedFile;
use rocket::response::Responder;
use rocket::{response, Request, Response};

struct DownloadFile {
    file: NamedFile,
    category: FileName,
    name: FileName,
    version: FileName,
}

impl<'r> Responder<'r> for DownloadFile {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.file.respond_to(req)?)
            .raw_header(
                "content-disposition",
                format!(
                    "attachment; filename=\"{}-{}-{}.tar.gz\"",
                    self.category.display(),
                    self.name.display(),
                    self.version.display()
                ),
            )
            .ok()
    }
}

#[get("/p/<category>/<name>/<version>/download")]
fn download(category: FileName, name: FileName, version: FileName) -> Option<DownloadFile> {
    let path = PathBuf::from(".")
        .join(&*RAVEN_REPOSITORY_PATH)
        .join(&category)
        .join(&name)
        .join(&version)
        .join("data")
        .with_extension("tar.gz");
    NamedFile::open(&path).ok().map(|file| DownloadFile {
        file,
        category,
        name,
        version,
    })
}
