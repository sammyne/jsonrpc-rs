use jsonrpc;
use jsonrpc::errors::Error;
use jsonrpc::server::{Server, Service};
use jsonrpc::transport::tcp::Transport;
use serde::{Deserialize, Serialize};

pub trait HelloWorldService {
    fn hello_world(request: Request) -> Result<Reply, Error>;
}

impl<T> Service for T
where
    T: HelloWorldService,
{
    fn do_request(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, Error> {
        todo!()
    }
}

pub struct HelloWorld {}

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub msg: String,
}

#[derive(Deserialize, Serialize)]
pub struct Reply {
    pub msg: String,
}

impl Service for HelloWorld {
    fn do_request(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, Error> {
        match method {
            "hello_world" => {
                let request: Request = serde_json::from_value(params)?;
                let reply = self.hello_world(&request)?;
                serde_json::to_value(&reply).map_err(Error::from)
            }
            _ => Err(Error::method_not_found()),
        }
    }
}

impl HelloWorld {
    pub fn hello_world(&self, request: &Request) -> Result<Reply, Error> {
        let reply = Reply {
            msg: format!("{} world", request.msg),
        };

        Ok(reply)
    }

    pub fn new() -> Self {
        Self {}
    }
}

fn main() {
    let t = Transport::new("127.0.0.1:9123").unwrap();

    let mut s = Server::new(t);

    let hello_world = HelloWorld::new();
    s.register_service("service", Box::new(hello_world))
        .unwrap();

    s.serve().unwrap();
}
