use std::sync::{Arc, RwLock};

use libnest::package::{PackageID, PackageShortName};
use rocket::State;
use rocket_contrib::templates::Template;
use serde_json::json;

use crate::config::Config;
use crate::package::NPFManager;
use crate::param::{CategoryNameParam, PackageNameParam, VersionParam};

#[get("/p/<category>/<name>/<version>/content")]
pub fn content(
    config: State<Arc<Config>>,
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    category: CategoryNameParam,
    name: PackageNameParam,
    version: VersionParam,
) -> Option<Template> {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    let id = PackageID::from(
        config.name().clone(),
        category.clone().into(),
        name.clone().into(),
        version.clone().into(),
    );

    let short_name = PackageShortName::from(category.clone().into(), name.clone().into());

    if let (Some(manifest), Ok(files)) = (
        npf_manager.manifest_of(&short_name),
        npf_manager.content_of(&id),
    ) {
        Some(Template::render(
            "pages/package/content",
            json!({
                "name": config.name(),
                "pretty_name": config.pretty_name(),
                "links": config.links(),
                "manifest": manifest,
                "files": files,
            }),
        ))
    } else {
        None
    }
}
