use std::collections::HashMap;
use std::pin::Pin;
use tokio::time::{Duration, timeout};

use crate::http::{HttpRequest, HttpResponse};

// TODO -> handlers response should impl Responder

pub trait Handler {
    type Future: Future<Output = Result<HttpResponse, anyhow::Error>> + Send;

    fn call(&mut self, request: HttpRequest) -> Self::Future;
}

#[derive(Clone)]
pub struct RequestHandler;

impl Handler for RequestHandler {
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        let response = HttpResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::new(),
        };

        Box::pin(async move {
            // TODO -> router logic here?

            Ok(response)
        })
    }
}

#[derive(Clone)]
pub struct Timeout<T> {
    duration: Duration,
    handler: T,
}

impl<T> Timeout<T> {
    pub fn new(duration: Duration, handler: T) -> Self {
        Self { duration, handler }
    }
}

impl<T> Handler for Timeout<T>
where
    T: Handler + Clone + Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        let mut this = self.clone();

        Box::pin(async move {
            let result = timeout(this.duration, this.handler.call(request)).await??;

            // TODO -> propper result pattern matching
            Ok(result)
        })
    }
}
