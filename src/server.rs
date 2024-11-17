use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn run(&mut self) {
        let axum_app: Router = Router::new()
            .route("/insert", get(insert))
            .route("/analyse", get(analyse));

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, axum_app).await.unwrap();
    }

    pub async fn insert(&mut self) -> &'static str {
        // TODO Decode the binary records.
        // TODO Hash to find the node.
        // TODO Insert records accordingly.

        "insert"
    }

    pub async fn analyse(&mut self) -> &'static str {
        // TODO Decode the binary records.
        // TODO Hash to find the node.
        // TODO Analyse records accordingly.

        "analyse"
    }
}

pub async fn insert() -> &'static str {
    // TODO Decode the binary records.
    // TODO Hash to find the node.
    // TODO Insert records accordingly.

    "insert"
}

pub async fn analyse() -> &'static str {
    // TODO Decode the binary records.
    // TODO Hash to find the node.
    // TODO Analyse records accordingly.

    "analyse"
}
