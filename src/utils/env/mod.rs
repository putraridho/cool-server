use std::env;

pub use error::{Error, Result};

mod error;

pub fn get_env(key: &'static str) -> Result<String> {
    env::var(key).map_err(|_| Error::MissingEnv(key))
}
