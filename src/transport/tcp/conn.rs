use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::transport::Conn;

pub struct TCPConn(pub(crate) TcpStream);

impl Conn for TCPConn {}

impl Read for TCPConn {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let length = {
            let mut length = [0u8; 8];
            self.0.read_exact(&mut length).map(|_| 0)?;
            u64::from_le_bytes(length) as usize
        };

        buf.resize(length, 0);
        self.0.read_exact(buf.as_mut_slice()).map(|_| 0)
    }
}

impl Write for TCPConn {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let length = (buf.len() as u64).to_le_bytes();
        let _ = self.0.write_all(&length)?;
        self.0.write_all(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn dial<A>(addr: A) -> Result<TCPConn, io::Error>
where
    A: ToSocketAddrs,
{
    Ok(TCPConn(TcpStream::connect(addr)?))
}
