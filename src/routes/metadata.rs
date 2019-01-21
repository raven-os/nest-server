use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Error;
use toml;

use crate::filename::FileName;
use crate::manifest::Manifest;
use crate::RAVEN_REPOSITORY_PATH;

#[get("/p/<category>/<name>/<version>/metadata")]
pub fn metadata(
    category: FileName,
    name: FileName,
    version: FileName,
) -> Result<rocket_contrib::json::Json<Manifest>, Error> {
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
    Ok(rocket_contrib::json::Json(toml::from_str(&s)?))
}
