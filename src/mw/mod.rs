use std::sync::Arc;

use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, to_value};
use surrealdb::sql::Uuid;
use tracing::debug;

use crate::{log::log_request, router};

pub async fn response_map(uri: Uri, req_method: Method, res: Response) -> Response {
    debug!("{:<12} -> mw::response_map", "MIDDLEWARE");
    let uuid = Uuid::new_v4();

    let router_error = res
        .extensions()
        .get::<Arc<router::Error>>()
        .map(Arc::as_ref);
    let client_status_error = router_error.map(|re| re.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error = to_value(client_error).ok();
            let message = client_error.as_ref().and_then(|v| v.get("message"));
            let detail = client_error.as_ref().and_then(|v| v.get("detail"));
            let client_error_body = json!({
              "error": {
                "message": message,
                "data": {
                  "req_uuid": uuid,
                  "detail": detail
                },
              }
            });

            debug!("-> CLIENT ERROR BODY:\n{client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    let _ = log_request(req_method, uri, router_error, client_error).await;

    debug!("\n");

    error_response.unwrap_or(res)
}
