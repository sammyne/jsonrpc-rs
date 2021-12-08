use jsonrpc::{rpcize, Metadata, Result, Service};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct Request {
    msg: String,
}

#[derive(Serialize, Deserialize)]
struct Reply {
    msg: String,
}

struct Hello {}

impl Hello {
    pub fn hello(&mut self, req: Request, metadata: &Metadata) -> Result<Reply> {
        let reply = Reply {
            msg: format!("hello: {}-{}", metadata.id.as_ref().unwrap(), req.msg),
        };

        Ok(reply)
    }

    pub fn hi(&mut self, req: Request, metadata: &Metadata) -> Result<String> {
        let id = metadata.id.as_ref().unwrap();
        let out = format!("hi: {}-{}", id, req.msg);
        Ok(out)
    }

    pub fn world(&mut self, _: Request, metadata: &Metadata) -> Result<i32> {
        //println!("world with id = {}", req.msg);
        let reply = i32::from_str_radix(metadata.id.as_ref().unwrap(), 10).unwrap();

        Ok(reply)
    }
}

rpcize!(Hello: "hello" | "hello1" -> hello, "world" -> world, hi);

fn main() {
    let mut h = Hello {};

    let params1: Value = json!({ "msg": "1234" });
    let metadata = Metadata {
        id: Some("7890".to_string()),
    };

    let methods = &["hello", "hello1", "world", "hi"];
    for v in methods {
        let reply = h.do_request(v, params1.clone(), &metadata).unwrap();
        println!("reply for '{}' = {}", v, reply);
    }
}
