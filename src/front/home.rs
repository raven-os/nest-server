use std::sync::{Arc, RwLock};

use rocket::State;
use rocket_contrib::templates::Template;
use serde_json::json;

use crate::config::Config;
use crate::package::NPFManager;

#[get("/")]
pub fn home(config: State<Arc<Config>>, npf_manager: State<Arc<RwLock<NPFManager>>>) -> Template {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    let manifests_count = npf_manager.manifests_count();
    let history = npf_manager.history().entries();

    Template::render(
        "pages/home",
        json!({
            "name": config.name(),
            "pretty_name": config.pretty_name(),
            "links": config.links(),
            "manifests_count": manifests_count,
            "history": history
        }),
    )
}
