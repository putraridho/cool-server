use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::migration::run_query;

use super::Result;

pub async fn define(db: &Surreal<Client>) -> Result<()> {
    let query = "
      DEFINE TABLE user SCHEMAFULL
        PERMISSIONS
          FOR select, update, delete WHERE id = $auth.id;
          
      DEFINE FIELD uid ON TABLE user TYPE string
        VALUE $before OR rand::uuid::v4()
        DEFAULT rand::uuid::v4();

      DEFINE FIELD username ON TABLE user TYPE string
        VALUE string::lowercase($value)
        ASSERT string::len($value) > 3;

      DEFINE INDEX userUsernameIndex ON TABLE user COLUMNS username UNIQUE;

      DEFINE FIELD email ON TABLE user TYPE string
        VALUE string::lowercase($value)
        ASSERT string::is::email($value);
        
      DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;

      DEFINE FIELD pwd ON TABLE user TYPE string
        PERMISSIONS FOR select NONE;

      DEFINE FIELD created_at ON TABLE user TYPE datetime
        VALUE $before OR time::now()
        DEFAULT time::now();

      DEFINE FIELD updated_at ON TABLE user TYPE datetime
        VALUE time::now()
        DEFAULT time::now();

      DEFINE FIELD deleted_at ON TABLE user TYPE option<datetime>
        DEFAULT NONE;
    ";

    run_query(db, query, "table_02_user").await
}
