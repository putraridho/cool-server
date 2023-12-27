use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::migration::run_query;

use super::Result;

pub async fn define(db: &Surreal<Client>) -> Result<()> {
    let query = "
        DEFINE SCOPE account SESSION 7d
          SIGNUP (
            CREATE user CONTENT {
              id: string::lowercase($username),
              username: $username,
              email: $email,
              pwd: crypto::argon2::generate($pwd)
            }
          )
          SIGNIN (
            SELECT * FROM user WHERE (
              (username = $email_or_username OR email = $email_or_username) AND
              crypto::argon2::compare(pwd, $pwd)
            )
          );
        ";

    run_query(db, query, "table_03_auth").await
}
