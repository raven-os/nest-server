use rocket::http::Status;

use crate::manifest::Manifest;
use crate::routes::search::search;

#[get("/pull")]
pub fn pull() -> Result<rocket_contrib::json::Json<Vec<Manifest>>, Status> {
    search(None)
}
