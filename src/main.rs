use axum::Extension;
use libsql::Builder;

// TODO:
// CRUD task
// Tile, Description and Status
// Filtering

mod routes;

#[tokio::main]
async fn main() {
    let db = Builder::new_local("local.db").build().await.unwrap();
    let conn = db.connect().unwrap();

    // Make a table it it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        title TEXT NOT NULL,
                        description TEXT,
                        status TEXT NOT NULL CHECK (status IN ('Todo', 'InProgress', 'Done'))
    )",
        (),
    )
    .await
    .unwrap();

    println!("statring the server...");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, routes::routers().layer(Extension(conn)))
        .await
        .unwrap();
}
