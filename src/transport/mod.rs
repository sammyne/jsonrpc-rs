use std::io::{Read, Write};
use std::iter::Iterator;

pub trait Conn: Read + Write {}

pub trait Transport {
    type C: Conn;

    fn connections(&mut self) -> Box<dyn Iterator<Item = std::io::Result<Self::C>> + '_>;
}

pub mod tcp;
