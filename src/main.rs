mod app;
mod config;
mod db;
mod node;
mod server;

use std::env;

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

    let _ = app::run(addrs).await;

    // TODO Create a channel to wait for app to exit.
}
