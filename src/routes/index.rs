use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use futures::stream::StreamExt;
use libsql::{de, Connection};
use minijinja::Environment;

use super::Task;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Tasks {
    tasks: Vec<Task>,
}

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

    let tasks = Tasks { tasks };

    let mut minijinja_env = Environment::new();
    let index_html = std::fs::read_to_string("index.html").unwrap();
    minijinja_env
        .add_template("index.html", &index_html)
        .unwrap();
    let tmpl = minijinja_env.get_template("index.html").unwrap();
    let html = tmpl.render(tasks).unwrap();

    (StatusCode::OK, Html(html))
}
