use anyhow::Result;
use bytes::BytesMut;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::Router;
use crate::http::{HTTPRequest, HTTPResponse, Method};

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

    pub async fn read(&mut self, router: Arc<Router>) -> Result<()> {
        let _advance = self.stream.read_buf(&mut self.read_buffer).await?;

        let request = self.parse_request()?;

        let tmp_route = &router.routes[0];
        let route_handler = &tmp_route.handler;

        let response = route_handler(request);

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
