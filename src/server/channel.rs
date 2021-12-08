use std::io;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::transport::Conn;

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

    pub fn recv_msg<T>(&mut self) -> io::Result<T>
    where
        T: DeserializeOwned,
    {
        let mut raw = vec![];
        let _ = self.conn.read_to_end(&mut raw)?;

        serde_json::from_slice(raw.as_slice())
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    pub fn send_msg<T>(&mut self, msg: &T) -> io::Result<()>
    where
        T: Serialize,
    {
        let msg_json = serde_json::to_vec(msg)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        self.conn.write_all(&msg_json)
    }
}
