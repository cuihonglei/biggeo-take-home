use axum::{routing::get, Router};
use std::env;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

struct DBNode {
    addr: String,
    stream: Option<TcpStream>,
}

impl DBNode {
    fn new(addr: String) -> DBNode {
        DBNode { addr, stream: None }
    }

    // Connect to the DB node.
    async fn connect(&mut self) -> Result<(), std::io::Error> {
        match TcpStream::connect(&self.addr).await {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    async fn insert(&mut self, data: Vec<u8>) {}

    async fn get_average(&mut self) {}
}

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

    // Establishe connections to the DB nodes
    let mut nodes = Vec::with_capacity(addrs.len());
    for addr in addrs {
        let mut node = DBNode::new(addr.to_string());

        if let Err(e) = node.connect().await {
            eprintln!("Failed to connect to {}: {}", addr, e);
            continue;
        }

        nodes.push(node);
    }

    // Print the nodes' addresses
    for node in nodes.iter() {
        println!("{}", node.addr);
    }

    // Start the API server.
    let app: Router = Router::new()
        .route("/insert", get(insert))
        .route("/analytics", get(analytics));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn insert() -> &'static str {
    // TODO Decode the binary records.

    // TODO Hash to find the node.

    // TODO Insert records accordingly.

    "insert"
}

async fn analytics() -> &'static str {
    // TODO Decode the binary records.

    // TODO Hash to find the node.

    // TODO Analyse records accordingly.

    "analytics"
}
