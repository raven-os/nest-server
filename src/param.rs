use libnest::package::{CategoryName, PackageName};
use semver::Version;

macro_rules! impl_into_value {
    ($Type:ident, $Value:ident) => {
        impl Into<$Value> for $Type {
            fn into(self) -> $Value {
                self.value
            }
        }
    };
}

macro_rules! impl_from_param {
    ($Type:ident, $Value:ident) => {
        impl<'a> ::rocket::request::FromParam<'a> for $Type {
            type Error = ::failure::Error;

            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<$Type, Self::Error> {
                let decoded_param = param.percent_decode()?;

                Ok($Type {
                    value: $Value::parse(decoded_param.as_ref())?,
                })
            }
        }
    };
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CategoryNameParam {
    value: CategoryName,
}

impl CategoryNameParam {
    pub fn value(&self) -> &CategoryName {
        &self.value
    }
}

impl_into_value!(CategoryNameParam, CategoryName);
impl_from_param!(CategoryNameParam, CategoryName);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageNameParam {
    value: PackageName,
}

impl PackageNameParam {
    pub fn value(&self) -> &PackageName {
        &self.value
    }
}

impl_into_value!(PackageNameParam, PackageName);
impl_from_param!(PackageNameParam, PackageName);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VersionParam {
    value: Version,
}

impl VersionParam {
    pub fn value(&self) -> &Version {
        &self.value
    }
}

impl_into_value!(VersionParam, Version);
impl_from_param!(VersionParam, Version);
