mod node_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/node_capnp.rs"));
}

mod app;
mod config;
mod db;
mod server;

use std::env;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    // Get the address and database path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <address> <db_path>", args[0]);
        std::process::exit(1);
    }

    let addr = &args[1];
    let db_path = &args[2];

    println!("DB node listening on {}", addr);
    println!("Using database at {}", db_path);

    let (tx, rx) = oneshot::channel();

    // Run the app.
    app::run(tx, addr, db_path).await;

    // Waiting for a signal to exit
    match rx.await {
        Ok(_) => {
            // Successfully received a signal to exit
            println!("Received exit signal, shutting down...");
        }
        Err(e) => {
            // If there's an error receiving the result
            eprintln!("Error waiting for exit signal: {:?}", e);
        }
    }

}
