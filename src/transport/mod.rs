use std::io::{self, Read, Write};

use std::net::SocketAddr;

pub trait Conn: Read + Write {
    fn local_addr(&self) -> io::Result<SocketAddr>;
    fn remote_addr(&self) -> io::Result<SocketAddr>;
}

// @ref: mirrors https://pkg.go.dev/net@go1.17.3#Listener
pub trait Listener {
    type C: Conn;

    fn addr(&self) -> io::Result<SocketAddr>;
    fn accept(&self) -> io::Result<Self::C>;

    //fn connections(&mut self) -> Box<dyn Iterator<Item = std::io::Result<Self::C>> + '_>;
}

pub mod tcp;
