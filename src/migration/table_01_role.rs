use crate::migration::run_query;
use surrealdb::{engine::remote::ws::Client, Surreal};

use super::Result;

pub async fn define(db: &Surreal<Client>) -> Result<()> {
    let query = "
      DEFINE TABLE role SCHEMAFULL;

      DEFINE FIELD name ON TABLE role TYPE string
        DEFAULT 'student';

      DEFINE FIELD created_at ON TABLE role TYPE datetime
        VALUE $before OR time::now()
        DEFAULT time::now();

      DEFINE FIELD updated_at ON TABLE role TYPE datetime
        VALUE time::now()
        DEFAULT time::now();
    ";

    run_query(db, query, "table_01_role").await
}
