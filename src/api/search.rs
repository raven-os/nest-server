use std::sync::{Arc, RwLock, RwLockReadGuard};

use libnest::package::PackageManifest;
use rocket::State;
use rocket_contrib::json::Json;

use crate::package::{ContentSearchResult, NPFManager};

#[get("/api/search?<q>&<exact_match>&search_by=content")]
pub fn search_content(
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    q: String,
    exact_match: Option<bool>,
) -> Json<Vec<ContentSearchResult>> {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    Json(
        npf_manager
            .container_of(&q, exact_match.unwrap_or_default())
            .unwrap_or_default(),
    )
}

pub fn do_search_metadata<'a>(
    npf_manager: &'a RwLockReadGuard<'_, NPFManager>,
    q: &str,
    search_by: &str,
    exact_match: bool,
) -> Vec<&'a PackageManifest> {
    if exact_match {
        npf_manager
            .manifests()
            .filter(|manifest| match search_by {
                "name" => manifest.name().as_ref() == q,
                "category" => manifest.category().as_ref() == q,
                "description" => manifest.metadata().description() == q,
                "tags" => manifest
                    .metadata()
                    .tags()
                    .iter()
                    .any(|tag| tag.as_ref() == q),
                _ => false,
            })
            .collect()
    } else {
        npf_manager
            .manifests()
            .filter(|manifest| match search_by {
                "name" => manifest.name().contains(&q),
                "category" => manifest.category().contains(&q),
                "description" => manifest.metadata().description().contains(&q),
                "tags" => manifest
                    .metadata()
                    .tags()
                    .iter()
                    .any(|tag| tag.contains(&q)),
                _ => false,
            })
            .collect()
    }
}

#[get("/api/search?<q>&<search_by>&<exact_match>")]
pub fn search_metadata(
    npf_manager: State<Arc<RwLock<NPFManager>>>,
    q: String,
    search_by: String,
    exact_match: Option<bool>,
) -> Json<Vec<PackageManifest>> {
    let npf_manager = npf_manager
        .read()
        .expect("can't open the NPF manager in read-only mode");

    Json(
        do_search_metadata(
            &npf_manager,
            &q,
            &search_by,
            exact_match.unwrap_or_default(),
        )
        .into_iter()
        .cloned()
        .collect(),
    )
}
