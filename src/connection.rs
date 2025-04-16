use anyhow::Result;
use bytes::BytesMut;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

/*
  Primary responsibility: Raw TCP I/O Management

  - Manage the TCP stream lifecycle
  - Buffer management for reading/writing
  - Handle low-level socket operations
  - Implement keep-alive mechanics
  - Handle TCP-level timeouts
  - Manage connection state (open, closing, closed)
  - Buffer size management
  - Handle partial reads/writes
*/

const READ_BUFF_CAPACITY: usize = 4 * 1024;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    read_buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        let read_buffer = BytesMut::with_capacity(READ_BUFF_CAPACITY);

        Connection {
            stream: BufWriter::new(socket),
            read_buffer,
        }
    }

    pub async fn read(&mut self) -> Result<Vec<u8>> {
        let _advance = self.stream.read_buf(&mut self.read_buffer).await?;
        let vec_res = self.read_buffer.to_vec();
        self.read_buffer.clear();

        Ok(vec_res)
    }

    pub async fn write(&mut self, response: &[u8]) -> Result<()> {
        self.stream.write_all(response).await?;
        self.stream.flush().await?;

        Ok(())
    }
}
