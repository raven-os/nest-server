use std::sync::{Arc, RwLock};

use rocket::State;
use rocket_contrib::json::JsonValue;
use serde_json::json;

use crate::config::Config;
use crate::package::NPFManager;

#[get("/api")]
pub fn home(config: State<Arc<Config>>, npf_manager: State<Arc<RwLock<NPFManager>>>) -> JsonValue {
    let npf_manager = npf_manager
        .read()
        .expect("cannot open the NPF manager in read-only mode");

    let manifests_count = npf_manager.manifests_count();
    let history = npf_manager.history().entries();

    let res = json!({
        "name": config.name(),
        "pretty_name": config.pretty_name(),
        "manifests_count": manifests_count,
        "history": history,
    });

    res.into()
}
