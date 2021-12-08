use std::iter::Iterator;
use std::net::{TcpListener, ToSocketAddrs};

use crate::transport::tcp::TCPConn;

pub struct TCPListener(TcpListener);

impl TCPListener {
    pub fn new<A>(addr: A) -> Result<Self, String>
    where
        A: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr).map_err(|err| err.to_string())?;

        Ok(Self(listener))
    }
}

impl crate::transport::Listener for TCPListener {
    type C = TCPConn;

    fn connections(&mut self) -> Box<dyn Iterator<Item = std::io::Result<Self::C>> + '_> {
        Box::new(self.0.incoming().map(|v| v.map(|s| TCPConn(s))))
    }
}
