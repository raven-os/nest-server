use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<String> {
    Html(format!(
        "Raven Stable Server v{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    ))
}
