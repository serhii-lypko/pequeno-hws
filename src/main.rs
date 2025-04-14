use std::collections::HashMap;
use std::marker::PhantomData;
use tokio::net::TcpListener;

use crate::error::AppResult;
use crate::http::{HTTPRequest, HTTPResponse};
use crate::server::Router;
use crate::server::run;

mod connection;
mod error;
mod http;
mod server;

/*
    TODO Middleware Integration:

    - HTTPRequest preprocessing
    - Authentication/authorization
    - Logging
    - HTTPResponse postprocessing
*/

// TODO: practice mpsc

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

trait Handler {
    type HTTPRequest;
    type HTTPResponse;
    type Error;

    fn call(&self, request: Self::HTTPRequest);
}

struct RouteHandler<F, RQ, RS>(F, PhantomData<(RQ, RS)>)
where
    F: Fn(RQ) -> RS;

impl<F, RQ, RS> Handler for RouteHandler<F, RQ, RS>
where
    F: Fn(RQ) -> RS,
    RQ: 'static,
    RS: 'static,
{
    type HTTPRequest = RQ;
    type HTTPResponse = RS;
    type Error = anyhow::Error;

    fn call(&self, request: Self::HTTPRequest) {
        todo!()
    }
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

struct Route2<F>
where
    F: Fn(HTTPRequest) -> HTTPResponse,
{
    path: String,
    handler: F,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    /*
        let app = Router::new()
            .route("/", get(root))
            .route("/users", post(create_user));
    */

    let handler = |req| HTTPResponse {
        status_code: 204,
        status_text: "No content".to_string(),
        headers: HashMap::new(),
    };

    let router = Router::new().route(Box::new(handler));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    run(listener, router).await?;

    Ok(())
}
