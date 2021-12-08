use jsonrpc;
use jsonrpc::client::Client;
use jsonrpc::transport::tcp;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Reply {
    pub msg: String,
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub msg: String,
}

fn main() {
    let mut c = {
        let conn = tcp::dial("127.0.0.1:9123").unwrap();
        Client::new(conn)
    };

    let params = Request {
        msg: "hello".to_string(),
    };
    let id = Some("1234567890".to_string());
    let request = jsonrpc::Request::new("service.hello_world", params, id);

    let reply: Reply = c.do_request(&request).unwrap();
    println!("reply = {}", reply.msg);
}
