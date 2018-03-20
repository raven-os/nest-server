// Rustc
#![warn(missing_debug_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
// Clippy
#![cfg_attr(feature = "cargo-clippy", warn(fallible_impl_from))]
#![cfg_attr(feature = "cargo-clippy", warn(int_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(mem_forget))]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(mutex_integer))]
#![cfg_attr(feature = "cargo-clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(used_underscore_binding))]
#![cfg_attr(feature = "cargo-clippy", warn(wrong_pub_self_convention))]
// Features
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod package;
mod routes;

fn main() {
    use rocket_contrib::Template;

    if let Err(e) = package::load_packages() {
        use std::process;

        eprintln!("error: {}", e);
        process::exit(1);
    }

    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::pull::pull,
                routes::download::download,
            ],
        )
        .attach(Template::fairing())
        .launch();
}
