use failure::Error;
use libnest::package::{CategoryName, PackageName};
use rocket::http::RawStr;
use rocket::request::FromParam;
use semver::Version;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CategoryNameParam {
    value: CategoryName,
}

impl CategoryNameParam {
    pub fn value(&self) -> &CategoryName {
        &self.value
    }
}

impl<'a> FromParam<'a> for CategoryNameParam {
    type Error = Error;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let decoded_param = param.percent_decode()?;

        Ok(CategoryNameParam {
            value: CategoryName::parse(decoded_param.as_ref())?,
        })
    }
}

impl Into<CategoryName> for CategoryNameParam {
    fn into(self) -> CategoryName {
        self.value
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageNameParam {
    value: PackageName,
}

impl PackageNameParam {
    pub fn value(&self) -> &PackageName {
        &self.value
    }
}

impl<'a> FromParam<'a> for PackageNameParam {
    type Error = Error;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let decoded_param = param.percent_decode()?;

        Ok(PackageNameParam {
            value: PackageName::parse(decoded_param.as_ref())?,
        })
    }
}

impl Into<PackageName> for PackageNameParam {
    fn into(self) -> PackageName {
        self.value
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VersionParam {
    value: Version,
}

impl VersionParam {
    pub fn value(&self) -> &Version {
        &self.value
    }
}

impl<'a> FromParam<'a> for VersionParam {
    type Error = Error;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let decoded_param = param.percent_decode()?;

        Ok(VersionParam {
            value: Version::parse(decoded_param.as_ref())?,
        })
    }
}

impl Into<Version> for VersionParam {
    fn into(self) -> Version {
        self.value
    }
}
