use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use futures::stream::StreamExt;
use libsql::{de, Connection};

use super::Task;

pub async fn all(Extension(db): Extension<Connection>) -> impl IntoResponse {
    let all_tasks = db
        .query("SELECT * FROM tasks", ())
        .await
        .unwrap()
        .into_stream();

    let tasks: Vec<Task> = all_tasks
        .filter_map(|row| async {
            match row {
                Ok(row) => Some(de::from_row::<Task>(&row).unwrap()),
                Err(_) => None,
            }
        })
        .collect()
        .await;

    (StatusCode::OK, Json(tasks))
}
