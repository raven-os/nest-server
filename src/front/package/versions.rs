use std::sync::{Arc, RwLock};

use libnest::package::PackageShortName;
use rocket::State;
use rocket_contrib::templates::Template;
use serde_json::json;

use super::PackageVersion;
use crate::config::Config;
use crate::package::NPFManager;
use crate::param::{CategoryNameParam, PackageNameParam};

#[get("/p/<category>/<name>/versions")]
pub fn versions(
    config: State<Arc<Config>>,
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    category: CategoryNameParam,
    name: PackageNameParam,
) -> Option<Template> {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    let short_name = PackageShortName::from(category.into(), name.into());

    if let Some(manifest) = npf_manager.manifest_of(&short_name) {
        let mut versions = manifest
            .versions()
            .clone()
            .into_iter()
            .map(|(version, metadata)| PackageVersion { version, metadata })
            .collect::<Vec<_>>();

        // Sort by versions
        versions.sort_by(|a, b| b.version.cmp(&a.version));

        Some(Template::render(
            "pages/package/versions",
            json!({
                "name": config.name(),
                "pretty_name": config.pretty_name(),
                "links": config.links(),
                "sorted_versions": versions,
                "manifest": manifest,
            }),
        ))
    } else {
        None
    }
}
