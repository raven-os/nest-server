use regex::Regex;

use rocket::http::RawStr;
use rocket::request::FromParam;
use rocket::response::NamedFile;

lazy_static! {
    static ref REGEX_CATEGORY_NAME: Regex = Regex::new(r"^[a-z\-]+$").unwrap();
    static ref REGEX_PACKAGE_NAME: Regex = Regex::new(r"^[a-z]+$").unwrap();
}

#[derive(Debug, Clone)]
pub struct PackageName(String);

#[derive(Debug, Clone)]
pub struct CategoryName(String);

impl<'a> FromParam<'a> for PackageName {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<PackageName, &'a RawStr> {
        let s = param.to_string();
        if REGEX_PACKAGE_NAME.is_match(&s) {
            Ok(PackageName(s))
        } else {
            Err(param)
        }
    }
}

impl<'a> FromParam<'a> for CategoryName {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<CategoryName, &'a RawStr> {
        let s = param.to_string();
        if REGEX_CATEGORY_NAME.is_match(&s) {
            Ok(CategoryName(s))
        } else {
            Err(param)
        }
    }
}

#[get("/download/<category>/<package>")]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn download(category: CategoryName, package: PackageName) -> Option<NamedFile> {
    let path = format!("packages/{}/{}.tar", category.0, package.0);
    NamedFile::open(&path).ok()
}
