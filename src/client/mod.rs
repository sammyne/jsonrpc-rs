use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::Error;
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

    pub fn do_request<S, R>(&mut self, request: &Request<S>) -> Result<R, Error>
    where
        S: Serialize,
        R: DeserializeOwned,
    {
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
}
