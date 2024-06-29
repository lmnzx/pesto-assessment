use axum::{http::StatusCode, response::IntoResponse, Extension};
use libsql::Connection;

pub async fn add_task(Extension(db): Extension<Connection>) -> impl IntoResponse {
    let mut stmt = db
        .prepare("INSERT INTO tasks (title, description, status) VALUES (?1, ?2, ?3)")
        .await
        .unwrap();

    stmt.execute(("Test Task", "This is a test", "Todo"))
        .await
        .unwrap();

    (StatusCode::OK, "added a new task")
}
