use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, ErrorKind, Read, Write};

use crate::errors::Error;
use crate::transport::Transport;
use crate::{Request, Response};

pub type HandleFunc = dyn Fn(Value) -> Result<Value, Error>;

pub struct Server<T>
where
    T: Transport,
{
    transport: T,
    services: HashMap<String, Box<dyn Service>>,
}

pub trait Service {
    fn do_request(&mut self, method: &str, params: Value) -> Result<Value, Error>;
}

impl<T> Server<T>
where
    T: Transport,
{
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            services: HashMap::new(),
        }
    }

    pub fn register_service(
        &mut self,
        name: &str,
        service: Box<dyn Service>,
    ) -> Result<(), String> {
        if self.services.contains_key(name) {
            return Err("name is occupied".to_string());
        }

        let _ = self.services.insert(name.to_string(), service);
        Ok(())
    }

    pub fn serve(&mut self) -> Result<(), io::Error> {
        for v in self.transport.connections() {
            let mut conn = Box::new(v?);

            let mut request_json = vec![];
            let _ = conn.read_to_end(&mut request_json)?;

            let request: Request<Value> = serde_json::from_slice(request_json.as_slice())?;

            // TODO: validate fields

            // find method
            let (service_name, method_name) = match request.method.as_str().split_once('.') {
                Some(v) => v,
                None => {
                    // TODO: feedback error
                    return Err(io::Error::new(ErrorKind::NotFound, "bad method name"));
                }
            };

            let service = match self.services.get_mut(service_name) {
                Some(v) => v,
                None => {
                    // TODO: feedback error
                    return Err(io::Error::new(ErrorKind::NotFound, "method not found"));
                }
            };

            let feedback = match service.do_request(method_name, request.params) {
                Ok(v) => {
                    let reply = Response::new_ok(v, request.id);
                    serde_json::to_vec(&reply).expect("fail to marshal response")
                }
                Err(err) => serde_json::to_vec(&err).expect("fail to marshal error"),
            };

            conn.write_all(&feedback)?;
        }

        unreachable!();
    }

    pub fn stop() -> Result<(), String> {
        todo!()
    }
}
