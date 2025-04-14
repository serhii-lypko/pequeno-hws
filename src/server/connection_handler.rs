use std::collections::HashMap;
use std::sync::Arc;

use crate::Router;
use crate::connection::Connection;
use crate::http::{HTTPRequest, HTTPResponse};

fn handler(req: HTTPRequest) -> HTTPResponse {
    HTTPResponse {
        status_code: 204,
        status_text: "No content".to_string(),
        headers: HashMap::new(),
    }
}

pub struct ConnectionHandler {
    connection: Connection,
    router: Arc<Router>,
    // shutdown: Shutdown,
    //  _shutdown_complete: mpsc::Sender<()>,
}

impl ConnectionHandler {
    pub fn new(connection: Connection, router: Arc<Router>) -> Self {
        ConnectionHandler { connection, router }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.connection.read(self.router.clone()).await?;

        Ok(())
    }
}
