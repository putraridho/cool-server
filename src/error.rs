use crate::{migration, model};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Migration(migration::Error),
    Model(model::Error),
}

impl From<migration::Error> for Error {
    fn from(v: migration::Error) -> Self {
        Self::Migration(v)
    }
}

impl From<model::Error> for Error {
    fn from(v: model::Error) -> Self {
        Self::Model(v)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
