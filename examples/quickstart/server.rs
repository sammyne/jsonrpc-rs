//#[macro_use]
//extern crate log;

use jsonrpc::server::Server;
use jsonrpc::transport::tcp::TCPListener;
use jsonrpc::{self, rpcize, Metadata, Result};
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
    pub fn hello_world(&mut self, request: Request, metadata: &Metadata) -> Result<Reply> {
        println!(
            "hello_world is called with id = {}",
            metadata.id.as_ref().unwrap()
        );
        let reply = Reply {
            msg: format!("{} world", request.msg),
        };

        Ok(reply)
    }

    pub fn notify(&mut self, request: Request, _metadata: &Metadata) -> Result<()> {
        println!(
            "notification with msg='{}' will feedback no reply",
            request.msg
        );
        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }
}

rpcize!(HelloWorld: hello_world, notify);

fn main() {
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let mut s = Server::new();

    let hello_world = HelloWorld::new();
    s.register_service("service", Box::new(hello_world))
        .unwrap();

    let t = TCPListener::new("127.0.0.1:9123").unwrap();

    log::info!("server is up :)");
    s.serve(t).unwrap();
}
