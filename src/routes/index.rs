use std::collections::HashMap;

use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    let version = format!("{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    );

    context.insert(String::from("version"), version);
    Template::render("index", context)
}
