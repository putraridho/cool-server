use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub use error::{Error, Result};
use tracing::info;

use crate::migration;

mod error;
pub mod user;

pub type Db = Surreal<Client>;

pub struct DbConfig<'a> {
    pub url: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub ns: &'a str,
    pub db_name: &'a str,
}

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new(config: DbConfig<'_>) -> Result<Self> {
        let db = new_db_pool(config).await?;

        Ok(Self { db })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }
}

pub async fn new_db_pool(config: DbConfig<'_>) -> Result<Db> {
    let DbConfig {
        url,
        username,
        password,
        ns,
        db_name,
    } = config;

    info!("{:<12} -> Initialize connection", "DB");
    let db = Surreal::new::<Ws>(url).await?;

    info!("{:<12} -> Signing in", "DB");
    db.signin(Root { username, password }).await?;

    info!("{:<12} -> Connecting database", "DB");
    db.use_ns(ns).use_db(db_name).await?;

    migration::run(&db).await?;

    Ok(db)
}
