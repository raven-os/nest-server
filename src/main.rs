#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate regex;

mod package;
mod routes;

fn main() {
    use rocket_contrib::Template;

    rocket::ignite()
        .mount("/", routes![
               routes::index::index,
               routes::pull::pull,
               routes::download::download,
        ])
        .attach(Template::fairing())
        .launch();
}
