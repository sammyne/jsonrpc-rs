use jsonrpc::server::Server;
use jsonrpc::transport::tcp::Transport;
use jsonrpc::{self, rpcize, Error, Metadata};
use serde::{Deserialize, Serialize};

pub struct HelloWorld {}

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub msg: String,
}

#[derive(Deserialize, Serialize)]
pub struct Reply {
    pub msg: String,
}

impl HelloWorld {
    pub fn hello_world(&mut self, request: Request, metadata: &Metadata) -> Result<Reply, Error> {
        println!(
            "hello_world is called with id = {}",
            metadata.id.as_ref().unwrap()
        );
        let reply = Reply {
            msg: format!("{} world", request.msg),
        };

        Ok(reply)
    }

    pub fn new() -> Self {
        Self {}
    }
}

rpcize!(HelloWorld: hello_world);

fn main() {
    let t = Transport::new("127.0.0.1:9123").unwrap();

    let mut s = Server::new(t);

    let hello_world = HelloWorld::new();
    s.register_service("service", Box::new(hello_world))
        .unwrap();

    s.serve().unwrap();
}
