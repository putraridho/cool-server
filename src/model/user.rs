use serde::{Deserialize, Serialize};
use surrealdb::{
    opt::auth::{Jwt, Scope},
    sql::{Datetime, Uuid},
};

use crate::config::config;

use super::{Error, ModelManager, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uid: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted_at: Option<Datetime>,
}

#[derive(Serialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub pwd: String,
}

#[derive(Serialize)]
pub struct UserUpdate {
    pub username: String,
    pub email: Option<String>,
    pub pwd: Option<String>,
}

#[derive(Serialize)]
pub struct UserLogin {
    pub email_or_username: String,
    pub pwd: String,
}

#[derive(Serialize)]
pub struct UserDelete {
    pub deleted_at: Option<Datetime>,
}

impl User {
    pub async fn create(mm: &ModelManager, user_c: UserCreate) -> Result<Jwt> {
        let config = config();
        let db = mm.db();

        let token = db
            .signup(Scope {
                namespace: &config.DB_NS,
                database: &config.DB_NAME,
                scope: "account",
                params: user_c,
            })
            .await?;

        Ok(token)
    }

    pub async fn list(mm: &ModelManager) -> Result<Vec<User>> {
        let db = mm.db();

        let res: Vec<User> = db.update("user").await?;

        Ok(res)
    }

    pub async fn me(mm: &ModelManager) -> Result<User> {
        let db = mm.db();

        let mut res = db
            .query("SELECT * FROM user WHERE username = $auth.username")
            .await
            .map_err(|_| Error::FailedQueryMe)?;

        if let Some(me) = res.take(0)? {
            Ok(me)
        } else {
            Err(Error::UserNotFound)
        }
    }

    pub async fn update(mm: &ModelManager, user_u: UserUpdate) -> Result<User> {
        let db = mm.db();

        let res: Option<User> = db
            .update(("user", &user_u.username.to_lowercase()))
            .content(user_u)
            .await?;

        let user = res.unwrap();

        Ok(user)
    }

    pub async fn delete(mm: &ModelManager, username: String) -> Result<User> {
        let db = mm.db();

        let res: Option<User> = db
            .update(("user", username.to_lowercase()))
            .content(UserDelete {
                deleted_at: Some(Datetime(chrono::Utc::now())),
            })
            .await?;

        let user = res.unwrap();

        Ok(user)
    }

    pub async fn login(mm: &ModelManager, user_l: UserLogin) -> Result<Jwt> {
        let config = config();
        let db = mm.db();

        let token = db
            .signin(Scope {
                namespace: &config.DB_NS,
                database: &config.DB_NAME,
                scope: "account",
                params: user_l,
            })
            .await?;

        Ok(token)
    }

    pub async fn logout(mm: &ModelManager) -> Result<()> {
        let db = mm.db();

        db.invalidate().await?;

        Ok(())
    }
}
