use anyhow::Result;
use bytes::BytesMut;
use std::collections::HashMap;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::http::{HTTPRequest, HTTPResponse, Method};

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

    pub async fn read(&mut self) -> Result<()> {
        let _advance = self.stream.read_buf(&mut self.read_buffer).await?;

        let request = self.parse_request()?;

        // let response = route_handler(request);

        let response = HTTPResponse {
            status_code: 204,
            status_text: "No content".to_string(),
            headers: HashMap::new(),
        };

        self.stream.write_all(&response.to_bytes()).await.unwrap();
        self.stream.flush().await?;

        Ok(())
    }

    fn parse_request(&self) -> Result<HTTPRequest> {
        let data = std::str::from_utf8(&self.read_buffer)?;
        let mut lines = data.split("\r\n");

        let request_line = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty request"))?;

        let mut parts = request_line.split_whitespace();

        let method: Method = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing method"))
            .and_then(|m| m.try_into())?;

        let route_path = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing route path"))?;

        let request = HTTPRequest {
            method,
            path: route_path.into(),
            headers: HashMap::new(),
        };

        Ok(request)
    }
}
