use serde_json::Value;
use std::collections::HashMap;
use std::io;

use crate::errors::{Error, Result};
use crate::transport::Conn;
use crate::transport::Listener;
use crate::{Metadata, Request, Response};

pub struct Server {
    services: HashMap<String, Box<dyn Service>>,
}

pub trait Service {
    fn do_request(&mut self, method: &str, params: Value, metadata: &Metadata) -> Result<Value>;
}

impl Server {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register_service(
        &mut self,
        name: &str,
        service: Box<dyn Service>,
    ) -> std::result::Result<(), String> {
        if self.services.contains_key(name) {
            return Err("name is occupied".to_string());
        }

        let _ = self.services.insert(name.to_string(), service);
        Ok(())
    }

    pub fn serve<L>(&mut self, listener: L) -> std::result::Result<(), io::Error>
    where
        L: Listener,
    {
        loop {
            let mut conn = listener.accept()?;

            loop {
                if let Err(err) = handle_request(&mut conn, &mut self.services) {
                    println!("fail to handle request: {:?}", err);
                    break;
                }
            }
        }
    }

    //pub fn stop() -> std::result::Result<(), String> {
    //    todo!()
    //}
}

fn handle_request<C>(conn: &mut C, services: &mut HashMap<String, Box<dyn Service>>) -> Result<()>
where
    C: Conn,
{
    let mut request_json = vec![];
    let _ = conn.read_to_end(&mut request_json)?;

    let request: Request<Value> =
        match serde_json::from_slice(request_json.as_slice()).map_err(Error::from) {
            Ok(v) => v,
            Err(err) => return feedback_err(conn, err).map_err(|err| err.wrap("parse request")),
        };

    // TODO: validate fields

    // find method
    let (service_name, method_name) = match request.method.as_str().split_once('.') {
        Some(v) => v,
        None => {
            let err = Error::method_not_found().wrap("method name must be '{service}.{method}'");
            return feedback_err(conn, err).map_err(|err| err.wrap("split service and method"));
        }
    };

    let service = match services.get_mut(service_name) {
        Some(v) => v,
        None => {
            let hint = format!("service={}", service_name);
            return feedback_err(conn, Error::method_not_found()).map_err(|err| err.wrap(&hint));
        }
    };

    let metadata = Metadata { id: request.id };
    let is_notification = metadata.id.is_none();

    let status = service.do_request(method_name, request.params, &metadata);
    if is_notification {
        return Ok(()); // drop response for notification
    }

    let reply = status.map_or_else(
        |err| Response::new_err(err),
        |ok| Response::new_ok(ok, metadata.id.unwrap()),
    );

    feedback_reply(conn, reply).map_err(|err| err.wrap("feedback response"))
}

fn feedback_err<T>(conn: &mut T, err: Error) -> Result<()>
where
    T: Conn,
{
    let r = Response::<Value>::new_err(err);
    feedback_reply(conn, r)
}

fn feedback_reply<T>(conn: &mut T, r: Response<Value>) -> Result<()>
where
    T: Conn,
{
    let reply_json = serde_json::to_vec(&r).map_err(Error::from)?;
    conn.write_all(&reply_json).map_err(Error::from)
}
