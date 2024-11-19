use crate::node_capnp::node;
use capnp::capability::Promise;

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
