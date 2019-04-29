use std::sync::Arc;

use rocket::Request;
use rocket::State;
use rocket_contrib::templates::Template;
use serde_json::json;

use crate::config::Config;

#[catch(404)]
pub fn not_found(request: &Request) -> Template {
    let config = request
        .guard::<State<Arc<Config>>>()
        .expect("can't retrieve the config state in error handler");

    Template::render(
        "pages/error",
        json!({
            "name": config.name(),
            "pretty_name": config.pretty_name(),
            "links": config.links(),
            "code": 404,
            "error": "Page Not Found",
        }),
    )
}

#[catch(403)]
pub fn forbidden(request: &Request) -> Template {
    let config = request
        .guard::<State<Arc<Config>>>()
        .expect("can't retrieve the config state in error handler");

    Template::render(
        "pages/error",
        json!({
            "name": config.name(),
            "pretty_name": config.pretty_name(),
            "links": config.links(),
            "code": 403,
            "error": "Forbidden",
        }),
    )
}

#[catch(500)]
pub fn internal_error(request: &Request) -> Template {
    let config = request
        .guard::<State<Config>>()
        .expect("can't retrieve the config state in error handler");

    Template::render(
        "pages/error",
        json!({
            "name": config.name(),
            "pretty_name": config.pretty_name(),
            "links": config.links(),
            "code": 500,
            "error": "Internal Server Error",
        }),
    )
}
