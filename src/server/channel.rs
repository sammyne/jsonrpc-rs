use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::transport::Conn;
use crate::{Error, Result};

pub struct Channel<C> {
    conn: C,
}

impl<C> Channel<C>
where
    C: Conn,
{
    pub fn new(conn: C) -> Self {
        Self { conn }
    }

    pub fn recv_msg<T>(&mut self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut raw = vec![];
        let _ = self.conn.read_to_end(&mut raw)?;

        serde_json::from_slice(raw.as_slice()).map_err(Error::from)
    }

    pub fn send_msg<T>(&mut self, msg: &T) -> Result<()>
    where
        T: Serialize,
    {
        let msg_json = serde_json::to_vec(msg).map_err(Error::from)?;
        self.conn.write_all(&msg_json).map_err(Error::from)
    }
}
