use std::sync::Arc;

use axum::{extract::State, routing::get, Router};
use tokio::net::TcpListener;

use super::app::App;
use super::db;

// Start the API server.
pub async fn run(app: Arc<App>) {
    let axum_app: Router = Router::new()
        .route("/insert", get(insert))
        .route("/get-average", get(get_average))
        .with_state(app);

    tokio::spawn(async {
        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, axum_app).await.unwrap();
    });
}

async fn insert(State(app): State<Arc<App>>) -> &'static str {
    // TODO Decode the binary records.
    // TODO Validate the input data.

    // TODO Call DB manger to insert data into databases.
    db::insert(&app.db, "test_key".to_string()).await;

    // TODO Return
    "insert"
}

async fn get_average(State(app): State<Arc<App>>) -> &'static str {
    // TODO Decode the binary records.
    // TODO Validate the input data.
     
    // TODO Call DB manger to search data frome databases.
    db::get_average(&app.db,"test_key".to_string()).await;

    // TODO Return
    "get-average"
}
