use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::transport::Connection;

pub struct LengthPrefixReadWriter {
    conn: TcpStream,
}

pub struct Transport {
    listener: TcpListener,
}

impl Read for LengthPrefixReadWriter {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.conn.read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let length = {
            let mut length = [0u8; 8];
            self.conn.read_exact(&mut length).map(|_| 0)?;
            u64::from_le_bytes(length) as usize
        };

        buf.resize(length, 0);
        self.conn.read_exact(buf.as_mut_slice()).map(|_| 0)
    }
}

impl Connection for LengthPrefixReadWriter {}

impl Write for LengthPrefixReadWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.conn.write(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let length = (buf.len() as u64).to_le_bytes();
        let _ = self.conn.write_all(&length)?;
        self.conn.write_all(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Transport {
    pub fn new<A>(addr: A) -> Result<Self, String>
    where
        A: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr).map_err(|err| err.to_string())?;

        Ok(Self { listener })
    }
}

impl crate::transport::Transport for Transport {
    type Conn = LengthPrefixReadWriter;

    fn connections(&mut self) -> Box<dyn Iterator<Item = std::io::Result<Self::Conn>> + '_> {
        Box::new(
            self.listener
                .incoming()
                .map(|v| v.map(|s| LengthPrefixReadWriter { conn: s })),
        )
    }
}

pub fn new_connection<A>(addr: A) -> Result<LengthPrefixReadWriter, io::Error>
where
    A: ToSocketAddrs,
{
    let out = LengthPrefixReadWriter {
        conn: TcpStream::connect(addr)?,
    };
    Ok(out)
}
