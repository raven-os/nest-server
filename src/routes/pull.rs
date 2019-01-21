use failure::Error;

use crate::manifest::Manifest;
use crate::routes::search::search;

#[get("/pull")]
pub fn pull() -> Result<rocket_contrib::json::Json<Vec<Manifest>>, Error> {
    search(None)
}
