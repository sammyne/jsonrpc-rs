use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::net::SocketAddr;

pub trait Conn: Read + Write {
    fn local_addr(&self) -> io::Result<SocketAddr>;
    fn remote_addr(&self) -> io::Result<SocketAddr>;
}

pub trait Listener {
    type C: Conn;

    fn connections(&mut self) -> Box<dyn Iterator<Item = std::io::Result<Self::C>> + '_>;
}

pub mod tcp;
