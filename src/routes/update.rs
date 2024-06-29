use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension};
use libsql::Connection;

use super::Status;

pub async fn update_task(
    Extension(db): Extension<Connection>,
    Query(state): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let id = match state.get("id") {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, "Missing id".to_string()),
    };

    let status_str = match state.get("status") {
        Some(status) => status,
        None => return (StatusCode::BAD_REQUEST, "Missing status".to_string()),
    };

    let status = match Status::from_str(status_str) {
        Ok(status) => status,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()),
    };

    let mut stmt = db
        .prepare("UPDATE tasks SET status = ?1 WHERE id = ?2")
        .await
        .unwrap();

    stmt.execute([status.to_str(), id]).await.unwrap();

    (StatusCode::OK, format!("task {id} updated to {status_str}"))
}
