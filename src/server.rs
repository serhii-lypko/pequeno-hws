use tokio::net::TcpListener;
use tokio::time::Duration;

use crate::connection::Connection;
use crate::connection_handler::ConnectionHandler;
use crate::request_handler::{RequestHandler, SimpleAuth, Timeout};

/*
  Primary responsibility: Connection Acceptance and Lifecycle Management

  - Accept incoming TCP connections
  - Manage server-wide resources and limits
  - Handle graceful shutdown
  - Configure global server settings (timeouts, max connections, etc.)
  - Spawn connection handlers for new connections
*/

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

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
            // TODO -> instead of direct listener accept, exponential backoff needs to be implemented
            // https://github.com/tokio-rs/mini-redis/blob/master/src/server.rs#L278
            let (tcp_stream, _socket_addr) = self.listener.accept().await?;

            let connection = Connection::new(tcp_stream);
            let mut connection_handler = ConnectionHandler::new(connection);

            // TODO -> where to place router logic?

            // Middlewares chain
            let req_handler = RequestHandler;
            // let timeout = Timeout::new(Duration::from_secs(5), req_handler);
            // let auth = SimpleAuth::new(timeout);

            // Spawn a new task to process the connections
            tokio::spawn(async move {
                if let Err(err) = connection_handler.run(req_handler).await {
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
