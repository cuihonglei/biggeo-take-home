use crate::node_capnp::node;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio_util::compat::TokioAsyncReadCompatExt;

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

pub type NodeTx = mpsc::Sender<Command>;

// The DB node, each for one db connection.
struct Node {
    addr: String,
    //client: node::Client,
}

pub async fn run(addr: String) -> Result<mpsc::Sender<Command>, Box<dyn std::error::Error>> {
    println!("node::run {}", addr);

    let addr1 = addr.clone();

    let local = tokio::task::LocalSet::new();
    local.run_until(async move {
        // Try connecting to the server
        let stream = TcpStream::connect(addr1).await.unwrap();
        stream.set_nodelay(true);

        let compat_stream = TokioAsyncReadCompatExt::compat(stream);
        let (reader, writer) = futures::io::AsyncReadExt::split(compat_stream);

        let rpc_network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));

        let mut rpc_system = RpcSystem::new(rpc_network, None);
        let client: node::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        tokio::task::spawn_local(rpc_system);

        // Send ...
        let mut request = client.insert_request();

        // Initialize and set the request data
        let msg = "something!";
        request.get().init_request().set_name(&msg[..]);

        // Await the response
        match request.send().promise.await {
            Ok(reply) => {
                // Safely access the nested fields with error handling
                let message = reply
                    .get()
                    .expect("Failed to get reply")
                    .get_reply()
                    .expect("Failed to get reply field")
                    .get_message()
                    .expect("Failed to get message")
                    .to_str()
                    .expect("Failed to convert message to string");

                println!("Received: {}", message);
            }
            Err(e) => {
                eprintln!("Error during RPC call: {:?}", e);
            }
        }
    }).await;

    let (tx, mut rx) = mpsc::channel(32);

    // Create the Node instance
    let node = Node { addr };

    // Start a task to handle commands asynchronously
    tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            use tokio::time::{sleep, Duration};
            use Command::*;

            match cmd {
                Insert { key, resp } => {
                    println!("Insert received! {}, {}", node.addr, key);

                    sleep(Duration::from_millis(2000)).await;

                    println!("Insert dealed! {}", key);

                    // TODO Call DB node server
                    on_insert(&node).await;

                    // TODO Return by using oneshot
                    let _ = resp.send(key);
                }
                GetAverage { key, resp } => {
                    println!("GetAverage received! {} {}", node.addr, key);

                    sleep(Duration::from_millis(2000)).await;

                    println!("GetAverage dealed! {}", key);

                    // TODO Call DB node server
                    on_get_average(&node).await;

                    // TODO Return by using oneshot
                    let _ = resp.send(key);
                }
            }
        }
    });

    // Return the sender so the caller can send commands to this task
    Ok(tx)
}

async fn on_insert(node: &Node) {
    // TODO Testing Cap’n Proto’s RPC
    /*
    let mut request = node.client.insert_request();

    // Initialize and set the request data
    let msg = ("something!");
    request.get().init_request().set_name(&msg[..]);

    // Send the request and handle the response
    let reply = request.send().promise.await.unwrap();

    println!(
        "received: {}",
        reply.get()?.get_reply()?.get_message()?.to_str()?
    );
    */
}

async fn on_get_average(node: &Node) {
    // TODO
}