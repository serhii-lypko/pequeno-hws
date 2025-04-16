use anyhow::Result;
use std::collections::HashMap;

use crate::connection::Connection;
use crate::http::{HTTPRequest, HTTPResponse, Method};

/*
    Primary responsibility: Connection Lifecycle and Protocol Flow

    - Orchestrate the request-response cycle
    - Manage connection state transitions
    - Coordinate between Connection (I/O) and RequestHandler (processing)
    - Handle connection-level errors
    - Implement backpressure mechanisms
    - Manage connection-specific timeouts
    - Handle protocol upgrades (if needed)
    - Clean up resources when connection ends
*/

/*
    Options:

    - Request handler owned by connection handler
    - Request handler shared via Arc
    - Request handler as trait object for flexibility
*/

pub struct ConnectionHandler {
    connection: Connection,
    // shutdown: Shutdown,
    //  _shutdown_complete: mpsc::Sender<()>,
}

impl ConnectionHandler {
    pub fn new(connection: Connection) -> Self {
        ConnectionHandler { connection }
    }

    pub async fn run<F, Fut>(&mut self, handler: F) -> anyhow::Result<()>
    where
        F: Fn(HTTPRequest) -> Fut,
        Fut: Future<Output = Result<HTTPResponse, anyhow::Error>>,
    {
        let raw_data = self.connection.read().await?;
        let parsed_req = self.parse_request(&raw_data)?;

        let response = handler(parsed_req).await?;

        self.connection.write(&response.to_bytes()).await?;

        Ok(())
    }

    fn parse_request(&self, raw_data: &[u8]) -> Result<HTTPRequest> {
        let data = std::str::from_utf8(raw_data)?;
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
