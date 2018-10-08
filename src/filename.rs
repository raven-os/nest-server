use std::ops::Deref;
use std::path::{Component, Path, PathBuf};
use std::str::Utf8Error;

use rocket::http::RawStr;
use rocket::request::FromParam;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FileNameError {
    Utf8Error(Utf8Error),
    InvalidPath,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FileName {
    value: PathBuf,
}

impl FileName {
    pub fn value(self) -> PathBuf {
        self.value
    }
}

impl AsRef<Path> for FileName {
    fn as_ref(&self) -> &Path {
        &self.value
    }
}

impl Deref for FileName {
    type Target = Path;

    fn deref(&self) -> &Path {
        &self.value
    }
}

impl<'a> FromParam<'a> for FileName {
    type Error = FileNameError;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let decoded_param = param
            .percent_decode()
            .map_err(|e| FileNameError::Utf8Error(e))?;
        let path = PathBuf::from(decoded_param.as_ref());
        if path.components().count() != 1 {
            Err(FileNameError::InvalidPath)
        } else {
            if let Some(Component::Normal(_)) = path.components().next() {
                Ok(FileName { value: path })
            } else {
                Err(FileNameError::InvalidPath)
            }
        }
    }
}
