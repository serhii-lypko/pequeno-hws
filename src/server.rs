use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::connection::Connection;
use crate::connection_handler::ConnectionHandler;
use crate::http::HttpResponse;

/*
  Primary responsibility: Connection Acceptance and Lifecycle Management

  - Accept incoming TCP connections
  - Manage server-wide resources and limits
  - Handle graceful shutdown
  - Configure global server settings (timeouts, max connections, etc.)
  - Spawn connection handlers for new connections
*/

struct Server {
    listener: TcpListener,
    // limit_connections: Arc<Semaphore>,
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Server { listener }
    }

    async fn run(&self) -> anyhow::Result<()> {
        loop {
            /*
                Server: accepts connections
                |-> handler_1 -> connection_1
                |-> handler_2 -> connection_2
                |-> ...

            */

            // TODO -> instead of direct listener accept, exponential backoff needs to be implemented
            // https://github.com/tokio-rs/mini-redis/blob/master/src/server.rs#L278
            let (tcp_stream, _socket_addr) = self.listener.accept().await?;

            /*
                Define handler outside of the spawn because:

                - Ownership and Move Semantics: By creating the handler outside the spawn, the code makes
                ownership transfer explicit. The handler instance is created in the parent task's context
                and then moved into the spawned task with async move. This clearly shows the transfer of resources.

                - Error Handling Before Task Creation: If there's an error during the creation of the Connection
                or initializing the Handler, it's handled at the listener level rather than inside each spawned task.
                This prevents spawning tasks that would immediately fail.

                - Resource Initialization Separation: It separates the concerns of resource acquisition
                (getting a socket, creating buffers) from the task of processing a connection. This makes
                the code more maintainable and easier to reason about.

                - Explicit Lifetime Management: By structuring the code this way, it's clear that the Handler
                (which contains the connection and other resources) lives exactly as long as the spawned task,
                and is dropped when the task completes.
            */
            let connection = Connection::new(tcp_stream);
            let mut connection_handler = ConnectionHandler::new(connection);

            // TODO -> handlers response should impl Responder

            let tmp_request_handler = |req| async {
                let mut response = HttpResponse {
                    status_code: 200,
                    // status_text: "No content".to_string(),
                    status_text: "OK".to_string(),
                    headers: HashMap::new(),
                };

                response.with_header();

                // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                Ok(response)
            };

            // Spawn a new task to process the connections
            tokio::spawn(async move {
                if let Err(err) = connection_handler.run(tmp_request_handler).await {
                    println!("Got error when running handler for connection {:?}", err);
                }
            });
        }
    }
}

pub async fn run(listener: TcpListener) -> anyhow::Result<()> {
    // TODO -> wait for shutdown signal

    let server = Server::new(listener);

    server.run().await?;

    Ok(())
}
