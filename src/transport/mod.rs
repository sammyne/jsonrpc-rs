use std::io::{Read, Write};
use std::iter::Iterator;

pub trait Connection: Read + Write {}

pub trait Transport {
    type Conn: Connection;

    fn connections(&mut self) -> Box<dyn Iterator<Item = std::io::Result<Self::Conn>> + '_>;
}

pub mod tcp;
