use rocket_contrib::Json;

use manifest::{Manifest, MANIFEST_LIST};

#[get("/pull")]
fn pull() -> Json<Vec<Manifest>> {
    Json(MANIFEST_LIST.clone())
}
