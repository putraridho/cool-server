use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use surrealdb::opt::auth::Jwt;
use tower_cookies::Cookies;

use crate::model::{
    user::{User, UserCreate, UserLogin, UserUpdate},
    ModelManager,
};

use super::{remove_token_cookie, set_token_cookie, Result};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/register", post(UserRouter::register))
        .route("/logout", post(UserRouter::logout))
        .route("/login", post(UserRouter::login))
        .route("/me", get(UserRouter::me))
        .route("/user", get(UserRouter::list))
        .route("/user", put(UserRouter::update))
        .route("/user", delete(UserRouter::delete))
        .with_state(mm)
}

struct UserRouter;

impl UserRouter {
    async fn register(
        State(mm): State<ModelManager>,
        cookies: Cookies,
        Json(payload): Json<RegisterPayload>,
    ) -> Result<Json<Jwt>> {
        let RegisterPayload {
            username,
            email,
            pwd,
        } = payload;
        let token = User::create(
            &mm,
            UserCreate {
                username,
                email,
                pwd,
            },
        )
        .await?;

        set_token_cookie(&cookies, token.as_insecure_token())?;

        Ok(Json(token))
    }

    async fn login(
        State(mm): State<ModelManager>,
        cookies: Cookies,
        Json(payload): Json<LoginPayload>,
    ) -> Result<Json<Jwt>> {
        let LoginPayload {
            email_or_username,
            pwd,
        } = payload;
        let token = User::login(
            &mm,
            UserLogin {
                email_or_username,
                pwd,
            },
        )
        .await?;

        set_token_cookie(&cookies, token.as_insecure_token())?;

        Ok(Json(token))
    }

    async fn logout(State(mm): State<ModelManager>, cookies: Cookies) -> Result<()> {
        User::logout(&mm).await?;

        remove_token_cookie(&cookies)?;
        Ok(())
    }

    async fn list(State(mm): State<ModelManager>) -> Result<Json<Vec<User>>> {
        let users = User::list(&mm).await?;

        Ok(Json(users))
    }

    async fn me(State(mm): State<ModelManager>) -> Result<Json<User>> {
        let me = User::me(&mm).await?;

        Ok(Json(me))
    }

    async fn update(
        State(mm): State<ModelManager>,
        Json(payload): Json<UpdatePayload>,
    ) -> Result<Json<User>> {
        let UpdatePayload {
            username,
            email,
            pwd,
        } = payload;

        let user = User::update(
            &mm,
            UserUpdate {
                username,
                email,
                pwd,
            },
        )
        .await?;

        Ok(Json(user))
    }

    async fn delete(
        State(mm): State<ModelManager>,
        Json(payload): Json<DeletePayload>,
    ) -> Result<Json<User>> {
        let user = User::delete(&mm, payload.username).await?;

        Ok(Json(user))
    }
}

#[derive(Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    pwd: String,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    email_or_username: String,
    pwd: String,
}

#[derive(Debug, Deserialize)]
struct UpdatePayload {
    username: String,
    email: Option<String>,
    pwd: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeletePayload {
    username: String,
}
