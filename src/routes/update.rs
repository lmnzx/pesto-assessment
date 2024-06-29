use std::collections::HashMap;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension,
};
use libsql::Connection;

use super::Status;

pub async fn update_task(
    Extension(db): Extension<Connection>,
    Query(state): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let id = match state.get("id") {
        Some(id) => id,
        None => return Redirect::to("/"),
    };

    let status_str = match state.get("status") {
        Some(status) => status,
        None => return Redirect::to("/"),
    };

    let status = match Status::from_str(status_str) {
        Ok(status) => status,
        Err(_) => return Redirect::to("/"),
    };

    let mut stmt = db
        .prepare("UPDATE tasks SET status = ?1 WHERE id = ?2")
        .await
        .unwrap();

    stmt.execute([status.to_str(), id]).await.unwrap();

    Redirect::to("/")
}
