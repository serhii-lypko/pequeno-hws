use tokio::net::TcpListener;

use crate::error::AppResult;
use crate::server::run;

mod connection;
mod error;
mod http;
mod server;

/*
    Middleware Integration:
    - Request preprocessing
    - Authentication/authorization
    - Logging
    - Response postprocessing

    Routing algorithm:
    - trie based pathfinding (prefix-trie)

*/

// TODO: practice mpsc

// TODO: would be very good to practice with:
// - redis and pub-sub stuff

#[tokio::main]
async fn main() -> AppResult<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    run(listener).await?;

    Ok(())
}
