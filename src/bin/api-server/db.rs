use tokio::sync::oneshot;

use super::node;

// The DB manager, manage db connections.
pub struct DB {
    // The nodes' transmitters.
    nodes: Vec<node::NodeTx>,
}

pub async fn run(addrs: &[String]) -> Result<DB, std::io::Error> {
    let mut db = DB { nodes: Vec::new() };

    // Establishe connections to the DB nodes
    for addr in addrs {
        // Try connecting to the node and get the tx (Sender)
        match node::run(addr.to_string()).await {
            Ok(tx) => {
                // On success, push the tx (sender) into nodes_tx
                db.nodes.push(tx);
            }
            Err(e) => {
                // On failure, print an error message and continue with the next node
                eprintln!("Failed to connect to {}: {}", addr, e);
                continue;
            }
        }
    }

    Ok(db)
}

pub async fn insert(db: &DB, key: String) {
    // TODO Hash to find the databases.

    let i = 0; // Hardcoded index, you would choose based on your logic
    let node = &db.nodes[i];

    // Create a oneshot sender to receive the response
    let (resp_tx, resp_rx) = oneshot::channel();

    // Send Insert command to the node
    let _ = node
        .send(node::Command::Insert { key, resp: resp_tx })
        .await;

    //  TODO Handle the response
    match resp_rx.await {
        Ok(response) => {
            println!("Insert response: {}", response);
        }
        Err(e) => {
            eprintln!("Error receiving insert response: {:?}", e);
        }
    }
}

pub async fn get_average(db: &DB, key: String) {
    // TODO Hash to find the databases.

    // Create a vector to store the receivers for each node
    let mut results = Vec::new();

    for node in &db.nodes {
        // Create a oneshot channel for each node to send back the result
        let (tx, rx) = oneshot::channel();

        // Send GetAverage command to the node
        let _ = node
            .send(node::Command::GetAverage {
                key: key.clone(),
                resp: tx,
            })
            .await;

        // Collect the receiver to get the results later
        results.push(rx);
    }

    // TODO Collect the results (await the receivers)
    for rx in results {
        match rx.await {
            Ok(result) => {
                println!("Received result: {}", result);
            }
            Err(e) => {
                eprintln!("Error receiving result: {:?}", e);
            }
        }
    }
}
