use std::collections::HashMap;

use anyhow::Result;
use bytes::{Buf, BytesMut};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{TcpListener, TcpStream},
};

use crate::connection::Connection;

use crate::http::{Request, Response};

struct RouteHandler {}

struct Router {}

// ---------------------- ---------------------- //

struct Server {
    listener: TcpListener,
    // limit_connections: Arc<Semaphore>,
}

struct ConnectionHandler {
    connection: Connection,
    // shutdown: Shutdown,
    //  _shutdown_complete: mpsc::Sender<()>,
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Server { listener }
    }

    async fn run(&self, router: Router) -> Result<()> {
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
            let mut handler = ConnectionHandler::new(connection);

            // Spawn a new task to process the connections
            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    println!("Got error when running handler for connection {:?}", err);
                }
            });
        }
    }
}

impl ConnectionHandler {
    fn new(connection: Connection) -> Self {
        ConnectionHandler { connection }
    }

    async fn run(&mut self) -> Result<()> {
        fn handler(req: Request) -> Response {
            dbg!(req);

            Response {
                status_code: 204,
                status_text: "No content".to_string(),
                headers: HashMap::new(),
            }
        }

        self.connection.read(handler).await?;

        Ok(())
    }
}

pub async fn run(listener: TcpListener) -> Result<()> {
    // TODO: wait for shutdown signal

    let server = Server::new(listener);

    let router = Router {};

    server.run(router).await?;

    Ok(())
}
