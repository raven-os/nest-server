use std::path::PathBuf;
use std::sync::Arc;

use libnest::package::PackageName;
use rocket::response::NamedFile;
use rocket::response::Responder;
use rocket::State;
use rocket::{response, Request, Response};
use semver::Version;

use crate::config::Config;
use crate::param::{CategoryNameParam, PackageNameParam, VersionParam};

#[derive(Debug)]
pub struct DownloadFile {
    file: NamedFile,
    name: PackageName,
    version: Version,
}

impl<'r> Responder<'r> for DownloadFile {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.file.respond_to(req)?)
            .raw_header(
                "content-disposition",
                format!(
                    "attachment; filename=\"{}-{}.nest\"",
                    self.name, self.version,
                ),
            )
            .ok()
    }
}

#[get("/api/p/<category>/<name>/<version>/download")]
pub fn download(
    config: State<Arc<Config>>,
    category: CategoryNameParam,
    name: PackageNameParam,
    version: VersionParam,
) -> Option<DownloadFile> {
    let path = PathBuf::from(config.package_dir())
        .join(category.value().as_ref())
        .join(name.value().as_ref())
        .join(format!(
            "{}-{}.nest",
            name.value().as_ref(),
            version.value()
        ));

    NamedFile::open(&path).ok().map(|file| DownloadFile {
        file,
        name: name.into(),
        version: version.into(),
    })
}
