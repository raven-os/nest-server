#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_blocks)]

#[macro_use]
extern crate rocket;

mod api;
mod config;
mod front;
mod package;
mod param;

use std::sync::{Arc, RwLock};

use failure::Error;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_cors::AllowedOrigins;

use crate::config::Config;
use crate::package::notify;
use crate::package::NPFManager;

fn main() -> Result<(), Error> {
    // Clean leftovers from previous runs
    package::clean_tmp_files()?;

    // Load the configuration
    let config = Arc::new(Config::load()?);

    // Create the NPF cache manager
    let mut npf_manager = NPFManager::new(config.clone());
    npf_manager.resync()?;
    let npf_manager = Arc::new(RwLock::new(npf_manager));

    // Asynchronously look for modifications on the file system
    notify::async_watch_fs(config.clone(), npf_manager.clone());

    let options = rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    };

    rocket::ignite()
        .mount(
            "/",
            routes![
                front::home::home,
                front::search::search,
                front::search::search_content,
                front::package::content::content,
                front::package::metadata::metadata,
                front::package::versions::versions,
                api::home::home,
                api::pull::pull,
                api::upload::upload,
                api::search::search_metadata,
                api::search::search_content,
                api::package::content::content,
                api::package::metadata::metadata,
                api::package::version::version,
                api::package::delete::delete,
                api::package::download::download,
            ],
        )
        .register(catchers![
            front::error::internal_error,
            front::error::forbidden,
            front::error::not_found,
        ])
        .mount("/css", StaticFiles::from("front/static/css"))
        .mount("/js", StaticFiles::from("front/static/js"))
        .mount("/img", StaticFiles::from("front/static/img"))
        .attach(options)
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("timeago", Box::new(front::hb::timeago));
            engines
                .handlebars
                .register_helper("concat", Box::new(front::hb::concat));
            engines
                .handlebars
                .register_helper("repository_name", Box::new(front::hb::repository_name));
            engines
                .handlebars
                .register_helper("category_name", Box::new(front::hb::category_name));
            engines
                .handlebars
                .register_helper("package_name", Box::new(front::hb::package_name));
            engines
                .handlebars
                .register_helper("capitalize", Box::new(front::hb::capitalize));
            engines
                .handlebars
                .register_helper("plural", Box::new(front::hb::plural));
        }))
        .manage(config)
        .manage(npf_manager)
        .launch();

    // Clean leftovers of current run
    package::clean_tmp_files()?;

    Ok(())
}
