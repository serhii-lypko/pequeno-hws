use std::collections::HashMap;

use crate::connection::Connection;
use crate::http::{HTTPRequest, HTTPResponse};

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

pub struct ConnectionHandler {
    connection: Connection,
    // shutdown: Shutdown,
    //  _shutdown_complete: mpsc::Sender<()>,
}

impl ConnectionHandler {
    pub fn new(connection: Connection) -> Self {
        ConnectionHandler { connection }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.connection.read().await?;

        Ok(())
    }
}
