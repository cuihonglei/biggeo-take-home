use super::config::{self, Config};
use super::db;
use super::server;

use std::sync::Arc;
use tokio::signal;
use tokio::sync::oneshot;

pub struct App {
    // The Global config
    pub config: Config,
}

impl App {
    pub fn check_signal(&self, tx: oneshot::Sender<i8>) {
        tokio::spawn(async {
            match signal::ctrl_c().await {
                Ok(()) => {
                    // TODO Check return?
                    let _ = tx.send(0);
                }
                Err(err) => {
                    eprintln!("Unable to listen for shutdown signal: {}", err);
                    // we also shut down in case of error
                }
            }
        });
    }
}

pub async fn run(tx: oneshot::Sender<i8>, addr: &String, db_path: &String) {
    // Load config.
    let config: Config = config::load();

    // TODO Open duckdb.
    // TODO Testing DuckDB - Pass the db_path from command line
    let _ = db::duckdb(db_path);

    // Create the app object.
    let app = Arc::new(App { config });

    // Start the RPC server.
    // Assuming that server::run doesn't return an error, you could log any issues inside it.
    // TODO Check return.
    let _ = server::run(addr).await;

    // Check kill signals to exit safely.
    app.check_signal(tx);
}
