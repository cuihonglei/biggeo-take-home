use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

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
    stream: TcpStream,
}

pub async fn run(addr: String) -> Result<mpsc::Sender<Command>, std::io::Error> {
    // Try connecting to the server
    let stream = TcpStream::connect(addr.clone()).await?;

    let (tx, mut rx) = mpsc::channel(32);

    // Create the Node instance
    let node = Node { addr, stream };

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

                    // TODO Return by using oneshot
                    let _ = resp.send(key);
                }
                GetAverage { key, resp } => {
                    println!("GetAverage received! {} {}", node.addr, key);

                    sleep(Duration::from_millis(2000)).await;

                    println!("GetAverage dealed! {}", key);

                    // TODO Call DB node server

                    // TODO Return by using oneshot
                    let _ = resp.send(key);
                }
            }
        }
    });

    // Return the sender so the caller can send commands to this task
    Ok(tx)
}
