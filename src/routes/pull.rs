use failure::Error;
use rocket_contrib::Json;

use manifest::Manifest;
use routes::search::search;

#[get("/pull")]
fn pull() -> Result<Json<Vec<Manifest>>, Error> {
    search()
}
