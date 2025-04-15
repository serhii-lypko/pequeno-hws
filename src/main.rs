use std::collections::HashMap;
use std::marker::PhantomData;
use tokio::net::TcpListener;

use crate::error::AppResult;
use crate::http::{HTTPRequest, HTTPResponse};
use crate::server::run;

mod connection;
mod connection_handler;
mod error;
mod http;
mod server;

/*
    TODO Middleware Integration:

    - HTTPRequest preprocessing
    - Authentication/authorization
    - Logging
    - HTTPResponse postprocessing

    TODO -> practice mpsc
*/

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

#[tokio::main]
async fn main() -> AppResult<()> {
    /*
        let app = Router::new()
            .route("/", get(root))
            .route("/users", post(create_user));
    */

    // let handler = |req| HTTPResponse {
    //     status_code: 204,
    //     status_text: "No content".to_string(),
    //     headers: HashMap::new(),
    // };

    // let router = Router::new().route(Box::new(handler));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    run(listener).await?;

    Ok(())
}
