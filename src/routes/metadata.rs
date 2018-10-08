use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Error;
use rocket_contrib::Json;
use toml;

use crate::filename::FileName;
use crate::manifest::Manifest;
use crate::RAVEN_REPOSITORY_PATH;

#[get("/p/<category>/<name>/<version>/metadata")]
fn metadata(
    category: FileName,
    name: FileName,
    version: FileName,
) -> Result<Json<Manifest>, Error> {
    let path = PathBuf::from(".")
        .join(&*RAVEN_REPOSITORY_PATH)
        .join(category)
        .join(name)
        .join(version)
        .join("manifest")
        .with_extension("toml");
    let mut file = File::open(path)?;
    let mut s = file
        .metadata()
        .map(|m| String::with_capacity(m.len() as usize))
        .unwrap_or_default();

    file.read_to_string(&mut s)?;
    Ok(Json(toml::from_str(&s)?))
}
