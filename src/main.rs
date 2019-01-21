// Rustc
#![warn(missing_debug_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![allow(elided_lifetimes_in_paths)] // disable warnings for rocket in rust 2018
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]
#[macro_use]
extern crate rocket;

use std::env;
use std::process;

use dotenv;
use lazy_static::lazy_static;
use rocket_cors::AllowedOrigins;

pub mod filename;
pub mod manifest;
pub mod routes;

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
    dotenv::dotenv().ok();
    lazy_static::initialize(&RAVEN_REPOSITORY_NAME);
    lazy_static::initialize(&RAVEN_REPOSITORY_PATH);

    let options = rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    };

    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::pull::pull,
                routes::search::search,
                routes::download::download,
                routes::metadata::metadata,
            ],
        )
        .attach(options)
        .launch();
}
