use std::env;
use tokio::io;

mod app;
use app::App;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("API server!");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <db_node1_addr> <db_node2_addr> ...", args[0]);
        std::process::exit(1);
    }

    // Read the addresses from the command-line arguments
    let addrs = &args[1..];

    App::new().run(addrs).await?;

    Ok(())
}
