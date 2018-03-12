use rocket_contrib::Json;

use package::{PACKAGE_LIST, Package};

#[get("/pull")]
fn pull() -> Json<Vec<Package>> {
    Json(PACKAGE_LIST.clone())
}
