use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use libsql::Connection;

pub async fn delete_task(
    Extension(db): Extension<Connection>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut stmt = db.prepare("DELETE FROM tasks WHERE id = ?1").await.unwrap();

    stmt.execute([id]).await.unwrap();

    (StatusCode::OK, format!("task {id} deleted"))
}
