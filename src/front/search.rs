use std::sync::{Arc, RwLock};

use rocket::State;
use rocket_contrib::templates::Template;
use serde_json::json;

use crate::api;
use crate::config::Config;
use crate::package::NPFManager;

#[get("/search?<q>&search_by=content")]
pub fn search_content(
    config: State<Arc<Config>>,
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    q: String,
) -> Template {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    // Quick & dirty fix to avoid huge results if the query is empty.
    // FIXME
    let results = {
        if q != "" {
            npf_manager
                .browse_packages_for_file(&q, false)
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    };

    Template::render(
        "pages/search/content",
        json!({
            "name": config.name(),
            "pretty_name": config.pretty_name(),
            "links": config.links(),
            "query": q,
            "search_by": "content",
            "results_len": results.len(),
            "results": results,
        }),
    )
}

#[get("/search?<q>&<search_by>")]
pub fn search(
    config: State<Arc<Config>>,
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    q: String,
    search_by: String,
) -> Template {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");
    let results = api::search::do_search_metadata(&npf_manager, &q, &search_by, false);

    Template::render(
        "pages/search/metadata",
        json!({
            "name": config.name(),
            "pretty_name": config.pretty_name(),
            "links": config.links(),
            "query": q,
            "search_by": search_by,
            "results_len": results.len(),
            "results": results,
        }),
    )
}
