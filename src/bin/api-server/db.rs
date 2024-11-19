use tokio::sync::{mpsc, oneshot};

use super::node;

pub enum Command {
    Insert {
        key: String,
        resp: oneshot::Sender<String>,
    },
    GetAverage {
        key: String,
        resp: oneshot::Sender<String>,
    },
}

pub type DBTx = mpsc::Sender<Command>;

// The DB manager, manage db connections.
pub struct DB {
    // The nodes' transmitters.
    nodes: Vec<node::NodeTx>,
}

pub async fn insert(db: &DBTx) {
    // TODO Testing

    // Create a oneshot channel for the response
    let (resp_tx, resp_rx) = oneshot::channel();

    // Create a Command::Insert with a test key and response sender
    let command = Command::Insert {
        key: "test_key".to_string(),
        resp: resp_tx,
    };

    // Send the command to the DB
    if let Err(e) = db.send(command).await {
        eprintln!("Failed to send command to DB: {}", e);
        return;
    }

    // Wait for the response
    match resp_rx.await {
        Ok(response) => println!("Received response from DB: {}", response),
        Err(e) => eprintln!("Failed to receive response from DB: {}", e),
    }
}

pub async fn get_average(db: &DBTx) {
    // TODO Testing

    // Create a oneshot channel for the response
    let (resp_tx, resp_rx) = oneshot::channel();

    // Create a Command::Insert with a test key and response sender
    let command = Command::GetAverage {
        key: "test_key".to_string(),
        resp: resp_tx,
    };

    // Send the command to the DB
    if let Err(e) = db.send(command).await {
        eprintln!("Failed to send command to DB: {}", e);
        return;
    }

    // Wait for the response
    match resp_rx.await {
        Ok(response) => println!("Received response from DB: {}", response),
        Err(e) => eprintln!("Failed to receive response from DB: {}", e),
    }
}

pub async fn run(addrs: &[String]) -> Result<DBTx, std::io::Error> {
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

    // Create a channel that will be used by the task to listen to commands
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a task to handle messages from outside the DB
    tokio::spawn(async move {
        // This task listens for commands and processes them
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Insert { key, resp } => {
                    // Handle Insert command
                    println!("Insert received for key: {}", key);

                    on_insert(&db, key).await;

                    // Simulate some DB operation
                    let _ = resp.send(format!("Inserted"));
                }
                GetAverage { key, resp } => {
                    // Handle GetAverage command
                    println!("GetAverage received for key: {}", key);

                    on_get_average(&db, key).await;

                    // Simulate some DB operation
                    let _ = resp.send(format!("GetAverage"));
                }
            }
        }
    });

    Ok(tx)
}

async fn on_insert(db: &DB, key: String) {
    // TODO Hash to find the databases.

    let i = 0; // Hardcoded index, you would choose based on your logic
    let node = &db.nodes[i];

    // Create a oneshot sender to receive the response
    let (resp_tx, resp_rx) = oneshot::channel();

    // Send Insert command to the node
    let _ = node
        .send(node::Command::Insert { key, resp: resp_tx })
        .await;

    // Handle the response
    match resp_rx.await {
        Ok(response) => {
            println!("Insert response: {}", response);
        }
        Err(e) => {
            eprintln!("Error receiving insert response: {:?}", e);
        }
    }
}

async fn on_get_average(db: &DB, key: String) {
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

    // Collect the results (await the receivers)
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
