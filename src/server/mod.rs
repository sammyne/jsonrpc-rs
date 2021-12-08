use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, ErrorKind, Read, Write};

use crate::errors::Result;
use crate::transport::Listener;
use crate::{Metadata, Request, Response};

pub struct Server<T>
where
    T: Listener,
{
    Listener: T,
    services: HashMap<String, Box<dyn Service>>,
}

pub trait Service {
    fn do_request(&mut self, method: &str, params: Value, metadata: &Metadata) -> Result<Value>;
}

impl<T> Server<T>
where
    T: Listener,
{
    pub fn new(Listener: T) -> Self {
        Self {
            Listener,
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

    pub fn serve(&mut self) -> std::result::Result<(), io::Error> {
        for v in self.Listener.connections() {
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

            let metadata = Metadata { id: request.id };
            let is_notification = metadata.id.is_none();

            let status = service.do_request(method_name, request.params, &metadata);
            if is_notification {
                continue; // drop response for notification
            }

            let id = metadata.id.unwrap();
            let feedback = {
                let feedback = status.map_or_else(
                    |err| Response::new_err(err, id.clone()),
                    |ok| Response::new_ok(ok, id.clone()),
                );
                match serde_json::to_vec(&feedback) {
                    Ok(v) => v,
                    Err(err) => {
                        println!("marshal response: {}", err);
                        continue;
                    }
                }
            };

            conn.write_all(&feedback)?;
        }

        unreachable!();
    }

    pub fn stop() -> std::result::Result<(), String> {
        todo!()
    }
}
