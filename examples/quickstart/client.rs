use jsonrpc::client::Client;
use jsonrpc::transport::tcp;
use jsonrpc::{self, transport::tcp::TCPConn};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Reply {
    pub msg: String,
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub msg: String,
}

fn do_request(c: &mut Client<TCPConn>) {
    let params = Request {
        msg: "hello".to_string(),
    };
    let id = Some("1234567890".to_string());
    let request = jsonrpc::Request::new("service.hello_world", params, id);

    let reply: Reply = c.do_request(&request).unwrap();
    println!("call method 'hello_world': reply = {}", reply.msg);
}

fn notify(c: &mut Client<TCPConn>) {
    let params = Request {
        msg: "you're late".to_string(),
    };

    let request = jsonrpc::Request::new("service.notify", params, None);
    match c.notify(&request) {
        Ok(_) => {}
        Err(err) => {
            println!("   code = {}", err.code);
            println!("message = {}", err.message);

            let data_ref = err.data.unwrap();
            let data = String::from_utf8_lossy(&data_ref);
            println!("   data = {}", data);

            unreachable!();
        }
    }
}

fn main() {
    {
        let mut c = {
            let conn = tcp::dial("127.0.0.1:9123").unwrap();
            Client::new(conn)
        };

        do_request(&mut c);
        notify(&mut c);
    }

    {
        let mut c = {
            let conn = tcp::dial("127.0.0.1:9123").unwrap();
            Client::new(conn)
        };

        do_request(&mut c);
        notify(&mut c);
    }
}
