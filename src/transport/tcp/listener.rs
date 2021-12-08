use std::io;
use std::net::{TcpListener, ToSocketAddrs};

use crate::transport::tcp::TCPConn;

pub struct TCPListener(TcpListener);

impl TCPListener {
    pub fn new<A>(addr: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        TcpListener::bind(addr).map(Self)
    }
}

impl crate::transport::Listener for TCPListener {
    type C = TCPConn;

    fn accept(&self) -> io::Result<Self::C> {
        self.0.accept().map(|(v, _)| TCPConn(v))
    }

    fn addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.0.local_addr()
    }
}
