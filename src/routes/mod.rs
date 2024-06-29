mod create;
mod delete;
mod index;
mod update;

use axum::{routing::get, Router};

use create::*;
use delete::*;
use index::*;
use update::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    id: u32,
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn from_str(s: &str) -> Result<Status, &'static str> {
        match s {
            "todo" => Ok(Status::Todo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err("Invalid status"),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "InProgress",
            Status::Done => "Done",
        }
    }
}

// Axum router
pub fn routers() -> Router {
    Router::new()
        .route("/create", get(add_task))
        .route("/", get(all))
        .route("/delete/:id", get(delete_task))
        .route("/update", get(update_task))
}
