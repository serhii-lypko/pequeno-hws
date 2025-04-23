use std::collections::HashMap;
use std::marker::PhantomData;
use tokio::net::TcpListener;

use crate::error::AppResult;
use crate::http::{HttpRequest, HttpResponse};
use crate::server::run;

mod connection;
mod connection_handler;
mod error;
mod http;
mod request_handler;
mod server;

/*
    Agenda:

    TODO -> routing

    TODO -> Middleware Integration:
    - Authentication/authorization
    - Retry
    - RateLimit?
    - Logging

    TODO -> graceful shutdown & connections limiting

    TODO -> practice mpsc
    https://tokio.rs/tokio/tutorial/channels
*/

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// Request -> middlewares 1 -> 2 .. -> N -> Router -> Result -> middlwares 3 -> ... N -> Response

#[tokio::main]
async fn main() -> AppResult<()> {
    /*
        let router = Router::new()
            .route("/health", get(health))
            .route("/test/room_socket_ids/{user_id}", get(get_room_socket_ids))
            .layer((
                CorsLayer::new().allow_origin(Any),
                TimeoutLayer::new(Duration::from_secs(REQUEST_TIMEOUT_SECS)),
            ))
            .with_state(shared_state);
    */

    // let handler = |req| HttpResponse {
    //     status_code: 204,
    //     status_text: "No content".to_string(),
    //     headers: HashMap::new(),
    // };

    // let router = Router::new().route(Box::new(handler));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    run(listener).await?;

    Ok(())
}
