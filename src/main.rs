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
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate glob;
extern crate semver;
extern crate toml;

pub mod manifest;
pub mod routes;

use std::env;
use std::process;

lazy_static! {
    static ref RAVEN_REPOSITORY_NAME: String = {
        if let Ok(s) = env::var("RAVEN_REPOSITORY_NAME") {
            s
        } else {
            eprintln!("error: the RAVEN_REPOSITORY_NAME variable is not set.");
            process::exit(1);
        }
    };
    static ref RAVEN_REPOSITORY_PATH: String = {
        if let Ok(s) = env::var("RAVEN_REPOSITORY_PATH") {
            s
        } else {
            eprintln!("error: the RAVEN_REPOSITORY_PATH variable is not set.");
            process::exit(1);
        }
    };
}

fn main() {
    lazy_static::initialize(&RAVEN_REPOSITORY_NAME);
    lazy_static::initialize(&RAVEN_REPOSITORY_PATH);

    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::pull::pull,
                routes::download::download,
            ],
        )
        .launch();
}
