use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::{Error, Result};
use crate::transport::Conn;
use crate::{Request, Response};

pub struct Client<T> {
    conn: T,
}

impl<T> Client<T>
where
    T: Conn,
{
    pub fn new(conn: T) -> Self {
        Self { conn }
    }

    pub fn do_request<S, R>(&mut self, request: &Request<S>) -> Result<R>
    where
        S: Serialize,
        R: DeserializeOwned,
    {
        if request.id.is_none() {
            return Err(Error::invalid_params().wrap("non-notification must have id"));
        }

        let request_json = serde_json::to_vec(request)?;
        self.conn.write_all(&request_json)?;

        let mut reply_json = vec![];
        self.conn.read_to_end(&mut reply_json)?;

        let reply: Response<R> = serde_json::from_slice(&reply_json).map_err(Error::from)?;

        match reply.result {
            Some(v) => Ok(v),
            None => Err(reply.error.unwrap()),
        }
    }

    pub fn notify<S>(&mut self, request: &Request<S>) -> Result<()>
    where
        S: Serialize,
    {
        if request.id.is_some() {
            return Err(Error::invalid_params().wrap("notification takes no id"));
        }

        let request_json = serde_json::to_vec(request)?;
        self.conn.write_all(&request_json)?;

        Ok(())
    }
}
