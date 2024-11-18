use super::config::{self, Config};
use super::db;
use super::server;

use std::sync::Arc;
use tokio::signal;
use tokio::sync::oneshot;

pub struct App {
    // The Global config
    pub config: Config,

    // The DB manager's transmitter
    pub db: db::DBTx,
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

pub async fn run(tx: oneshot::Sender<i8>, addrs: &[String]) {
    // Load config.
    let config: Config = config::load();

    // Run the DB manager and get the transmitter.
    let db = match db::run(addrs).await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error running DB manager: {}", e);
            return;  // Return early if DB initialization fails
        }
    };

    // Create the app object.
    let app = Arc::new(App { config, db });

    // Start the API server.
    // Assuming that server::run doesn't return an error, you could log any issues inside it.
    server::run(app.clone()).await;

    // Check kill signals to exit safely.
    app.check_signal(tx);
}
