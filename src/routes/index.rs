use rocket::response::content::Html;

use crate::RAVEN_REPOSITORY_NAME;

#[get("/")]
pub fn index() -> Html<String> {
    Html(format!(
        "Raven \"{}\" v{}.{}.{}",
        *RAVEN_REPOSITORY_NAME,
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    ))
}
