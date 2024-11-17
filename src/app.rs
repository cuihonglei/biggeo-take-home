use std::io;

// TODO Best practice for sibling modules?
#[path = "config.rs"]
mod config;
use config::Config;

// TODO Best practice for sibling modules?
#[path = "db.rs"]
mod db;
use db::DB;

// TODO Best practice for sibling modules?
#[path = "server.rs"]
mod server;
use server::Server;

pub struct App {
    // The Global config
    config: Config,

    // The API server
    server: Server,

    // The DB manager
    db: DB,
}

impl App {
    pub fn new() -> App {
        App {
            config: Config::new(),
            server: Server::new(),
            db: DB::new(),
        }
    }

    pub async fn run(&mut self, addrs: &[String]) -> io::Result<()> {

        // Load config from file.
        self.config.load("");

        // Connect to DB nodes.
        self.db.run(addrs).await?;

        // Start the API server.
        self.server.run().await;

        Ok(())
    }
}
