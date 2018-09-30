use failure::Error;
use rocket_contrib::Json;

use crate::manifest::Manifest;
use crate::routes::search::search;

#[get("/pull")]
fn pull() -> Result<Json<Vec<Manifest>>, Error> {
    search()
}
