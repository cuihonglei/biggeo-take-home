mod app;
mod config;
mod db;
mod node;
mod server;

use std::env;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    println!("API server!");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <db_node1_addr> <db_node2_addr> ...", args[0]);
        std::process::exit(1);
    }

    // Read the addresses from the command-line arguments
    let addrs = &args[1..];

    let (tx, rx) = oneshot::channel();

    // Run the app.
    app::run(tx, addrs).await;

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
