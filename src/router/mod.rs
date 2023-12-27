mod error;
mod routes_user;

use axum::Router;
pub use error::{ClientError, Error, Result};
use tower_cookies::{Cookie, Cookies};

use crate::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().merge(routes_user::routes(mm.clone()))
}

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, token: &str) -> Result<()> {
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}
