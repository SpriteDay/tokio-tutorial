use bytes::BytesMut;
use mini_redis::{Frame, Result};
use tokio::net::TcpStream;

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            // Allocated buffer with 4kb of capacity
            buffer: BytesMut::with_capacity(4096),
        }
    }
    /// Read a frame from connection
    ///
    /// Returns 'None' if EOF is reached
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        Ok(None)
    }

    /// Write a frame to the connection
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {}
