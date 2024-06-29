use axum::{
    response::{IntoResponse, Redirect},
    Extension, Form,
};
use libsql::Connection;

#[derive(serde::Deserialize)]
pub struct Input {
    title: String,
    description: String,
}

pub async fn add_task(
    Extension(db): Extension<Connection>,
    Form(input): Form<Input>,
) -> impl IntoResponse {
    let mut stmt = db
        .prepare("INSERT INTO tasks (title, description, status) VALUES (?1, ?2, ?3)")
        .await
        .unwrap();

    stmt.execute((input.title, input.description, "Todo"))
        .await
        .unwrap();

    Redirect::to("/")
}
