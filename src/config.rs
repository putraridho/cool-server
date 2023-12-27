use std::sync::OnceLock;

use crate::utils::env::{get_env, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub DB_URL: String,
    pub DB_USERNAME: String,
    pub DB_PASSWORD: String,
    pub DB_NS: String,
    pub DB_NAME: String,
    pub SERVICE_HOST: String,
    pub SERVICE_PORT: String,
}
impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            DB_URL: get_env("DB_URL")?,
            DB_USERNAME: get_env("DB_USER")?,
            DB_PASSWORD: get_env("DB_PASSWORD")?,
            DB_NS: get_env("DB_NS")?,
            DB_NAME: get_env("DB_NAME")?,

            SERVICE_HOST: get_env("SERVICE_HOST")?,
            SERVICE_PORT: get_env("SERVICE_PORT")?,
        })
    }
}
