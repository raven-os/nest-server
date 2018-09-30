use std::fs::File;
use std::io::Read;

use failure::Error;
use glob;
use rocket_contrib::Json;
use toml;

use crate::manifest::Manifest;
use crate::RAVEN_REPOSITORY_PATH;

#[get("/info/<category>/<name>/<version>")]
pub fn info(category: String, name: String, version: String) -> Result<Json<Vec<Manifest>>, Error> {
    let mut manifests = Vec::new();
    for path in glob::glob(&format!("{}/**/*.toml", *RAVEN_REPOSITORY_PATH))? {
        let path = path?;
        let mut file = File::open(path)?;

        // Allocate a string long enough to hold the entire file
        let mut s = file
            .metadata()
            .map(|m| String::with_capacity(m.len() as usize))
            .unwrap_or_default();

        file.read_to_string(&mut s)?;
        manifests.push(toml::from_str(&s)?);
    }
    manifests.retain(|ref x: &Manifest| {
        x.metadata().category() == category.as_str()
            && x.metadata().name() == name.as_str()
            && x.metadata().version().to_string() == version
    });
    Ok(Json(manifests))
}
