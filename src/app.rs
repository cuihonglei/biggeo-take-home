use super::config::{self, Config};
use super::db;
use super::server;

use std::sync::Arc;

pub struct App {
    // The Global config
    pub config: Config,

    // The DB manager's transmitter
    pub db: db::DBTx,
}

impl App {}

pub async fn run(addrs: &[String]) -> Result<Arc<App>, std::io::Error> {
    // TODO Load config.
    let config = config::load();

    // Run the DB manager and get the transmitter.
    let db = db::run(addrs).await?;

    let app = Arc::new(App { config, db });

    // Start the API server.
    server::run(app.clone()).await;

    Ok(app)
}
