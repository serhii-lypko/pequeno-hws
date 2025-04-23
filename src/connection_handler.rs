use anyhow::Result;
use std::collections::HashMap;
use tokio::time::{Duration, timeout};

use crate::connection::Connection;
use crate::http::{HttpRequest, HttpResponse, Method, response::ResponseTrait};

use crate::request_handler::Service;

pub struct ConnectionHandler {
    connection: Connection,
    // shutdown: Shutdown,
    //  _shutdown_complete: mpsc::Sender<()>,
}

impl ConnectionHandler {
    pub fn new(connection: Connection) -> Self {
        ConnectionHandler { connection }
    }

    /*
        TODO -> how to define different kinds of response

        TODO -> so the response value should be "buildable"

        #[post("/echo")]
        async fn echo(req_body: String) -> impl Responder {
            HttpResponse::Ok().body(req_body)
        }

        async fn manual_hello() -> impl Responder {
            HttpResponse::Ok().body("Hey there!")
        }
    */

    pub async fn run<S>(&mut self, mut handler: S) -> anyhow::Result<()>
    where
        S: Service,
    {
        let raw_data = self.connection.read().await?;
        let parsed_req = self.parse_request(&raw_data)?;

        let response_result = handler.call(parsed_req).await;

        if let Ok(response) = response_result {
            self.connection.write(&response.to_bytes()).await?;
        }

        Ok(())
    }

    fn parse_request(&self, raw_data: &[u8]) -> Result<HttpRequest> {
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

        let mut headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }

            if let Some((name, value)) = line.split_once(": ") {
                headers.insert(name.trim().to_string(), value.trim().to_string());
            }
        }

        let request = HttpRequest {
            method,
            path: route_path.into(),
            headers,
        };

        Ok(request)
    }
}
