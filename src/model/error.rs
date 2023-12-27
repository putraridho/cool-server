use serde::Serialize;

use crate::migration;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    FailedQueryMe,
    UserNotFound,

    Surrealdb(surrealdb::Error),
    Migration(migration::Error),
}

impl From<migration::Error> for Error {
    fn from(v: migration::Error) -> Self {
        Self::Migration(v)
    }
}

impl From<surrealdb::Error> for Error {
    fn from(v: surrealdb::Error) -> Self {
        Self::Surrealdb(v)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
