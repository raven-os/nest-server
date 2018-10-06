use std::path::PathBuf;

use crate::filename::FileName;
use crate::RAVEN_REPOSITORY_PATH;

use rocket::response::NamedFile;
use rocket::response::Responder;
use rocket::{response, Request, Response};

struct DownloadFile(NamedFile, FileName, FileName, FileName);

impl<'r> Responder<'r> for DownloadFile {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header(
                "content-disposition",
                "attachment; filename=\"".to_owned()
                    + self
                        .1
                        .value()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .as_str()
                    + "-"
                    + self
                        .2
                        .value()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .as_str()
                    + "-"
                    + self
                        .3
                        .value()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .as_str()
                    + ".tar.gz"
                    + "\"",
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
    NamedFile::open(&path)
        .ok()
        .map(|nf| DownloadFile(nf, category, name, version))
}
