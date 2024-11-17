use std::io;

use tokio::net::TcpStream;

// The DB node, each for one db connection.
struct Node {
    addr: String,
    stream: Option<TcpStream>,
}

impl Node {
    fn new(addr: String) -> Node {
        Node { addr, stream: None }
    }

    // Connect to the DB node.
    pub async fn connect(&mut self) -> Result<(), std::io::Error> {
        match TcpStream::connect(&self.addr).await {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn insert(&mut self, data: Vec<u8>) {}

    pub async fn get_average(&mut self) {}
}

// The DB manager, manage db connections.
pub struct DB {
    nodes: Vec<Node>,
}

impl DB {
    pub fn new() -> DB {
        DB {
            // TODO Use the capacity from the input node count.
            nodes: Vec::with_capacity(2),
        }
    }
    
    pub async fn run(&mut self, addrs: &[String]) -> io::Result<()> {
        
        // Establishe connections to the DB nodes
        for addr in addrs {
            let mut node = Node::new(addr.to_string());

            if let Err(e) = node.connect().await {
                eprintln!("Failed to connect to {}: {}", addr, e);
                continue;
            }

            self.nodes.push(node);
        }

        // Print the nodes' addresses
        for node in self.nodes.iter() {
            println!("{}", node.addr);
        }

        Ok(())
    }

}
