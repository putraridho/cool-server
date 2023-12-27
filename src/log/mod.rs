use axum::http::{Method, Uri};
use chrono::Utc;
use serde::Serialize;
use serde_json::{json, to_value, Value};
use surrealdb::sql::Uuid;
use tracing::debug;

use crate::{router, Result};

pub async fn log_request(
    http_method: Method,
    uri: Uri,
    router_error: Option<&router::Error>,
    client_error: Option<router::ClientError>,
) -> Result<()> {
    let error_type = router_error.map(|re| re.to_string());
    let error_data = to_value(router_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: Uuid::new_v4().to_string(),
        timestamp: Utc::now().to_rfc3339(),
        http_path: uri.to_string(),
        http_method: http_method.to_string(),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    debug!("REQUEST LOG LINE:\n{}", json!(log_line));

    Ok(())
}

#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,
    http_path: String,
    http_method: String,
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
