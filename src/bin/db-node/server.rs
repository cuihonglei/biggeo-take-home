use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::node_capnp::node;

pub struct NodeImpl;

impl node::Server for NodeImpl {
    fn insert(
        &mut self,
        params: node::InsertParams,
        mut results: node::InsertResults,
    ) -> Promise<(), ::capnp::Error> {
        let name = params
            .get()
            .unwrap()
            .get_request()
            .unwrap()
            .get_name()
            .unwrap()
            .to_str()
            .unwrap();

        println!("node::Server::insert {}", name);

        let message = format!("Hello, {name}!");
        results.get().init_reply().set_message(message.as_str());

        Promise::ok(())
    }
}

pub async fn run(addr: &String) -> Result<(), Box<dyn std::error::Error>> {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let node_client: node::Client = capnp_rpc::new_client(NodeImpl);

            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;

                let compat_stream = TokioAsyncReadCompatExt::compat(stream);

                let (reader, writer) = futures::io::AsyncReadExt::split(compat_stream);

                let network = twoparty::VatNetwork::new(
                    futures::io::BufReader::new(reader),
                    futures::io::BufWriter::new(writer),
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system =
                    RpcSystem::new(Box::new(network), Some(node_client.clone().client));

                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}
