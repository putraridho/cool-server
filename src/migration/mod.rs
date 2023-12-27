pub use error::{Error, Result};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::{error, info};

use crate::model::Db;

mod error;
mod table_01_role;
mod table_02_user;
mod table_03_auth;

pub async fn run(db: &Surreal<Client>) -> Result<()> {
    info!("{:<12} -> Running Migration", "MIGRATION");

    table_01_role::define(db).await?;
    table_02_user::define(db).await?;
    table_03_auth::define(db).await?;

    Ok(())
}

pub async fn run_query(db: &Db, query: &'static str, migration_name: &'static str) -> Result<()> {
    info!(
        "{:<12} -> Running migration => {migration_name}",
        "MIGRATION"
    );

    db.query(query).await.map_err(|e| {
        error!(
            "{:<12} -> Error while running migration => {e:?}",
            "MIGRATION"
        );
        Error::BadMigration(migration_name)
    })?;

    Ok(())
}
