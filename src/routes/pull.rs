use rocket_contrib::Json;

use package::{Package, PACKAGE_LIST};

#[get("/pull")]
fn pull() -> Json<Vec<Package>> {
    Json(PACKAGE_LIST.clone())
}
