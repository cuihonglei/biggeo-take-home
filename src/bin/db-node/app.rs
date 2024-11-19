use super::config::{self, Config};
use super::db;
use super::server;

use std::sync::Arc;
use tokio::signal;
use tokio::sync::oneshot;

pub struct App {
    // The Global config
    pub config: Config,

    // The DB object
    pub db: db::DB,
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

    // Open DuckDB and create the `DB` object.
    let db = match db::run(db_path) {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error initializing the database: {}", err);
            return;
        }
    };

    // Create the app object.
    let app: Arc<App> = Arc::new(App { config, db });

    // Start the RPC server.
    // Assuming that server::run doesn't return an error, you could log any issues inside it.
    // TODO Check return.
    let _ = server::run(app.clone(), addr).await;

    // Check kill signals to exit safely.
    app.check_signal(tx);
}
