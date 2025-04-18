use std::pin::Pin;
use tokio::time::{Duration, timeout};

use crate::http::{
    HttpRequest, HttpResponse,
    response::{IntoResponse, StatusCode},
};

// TODO -> how to make Handler operate over IntoResponse?
pub trait Handler {
    // TODO -> so how to make it return IntoResponse?
    type Future: Future<Output = Result<HttpResponse, anyhow::Error>> + Send;

    fn call(&mut self, request: HttpRequest) -> Self::Future;
}

#[derive(Clone)]
pub struct RequestHandler;

impl Handler for RequestHandler {
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        // TODO -> need to handle router somehow

        let status_code = StatusCode::OK;
        let resp = status_code.into_response();

        Box::pin(async move { Ok(resp) })
    }
}

#[derive(Clone)]
pub struct Timeout<T> {
    duration: Duration,
    handler: T,
}

impl<T> Timeout<T> {
    pub fn new(duration: Duration, handler: T) -> Self {
        Timeout { duration, handler }
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

#[derive(Clone)]
pub struct SimpleAuth<T> {
    handler: T,
}

impl<T> SimpleAuth<T> {
    pub fn new(handler: T) -> Self {
        SimpleAuth { handler }
    }
}

impl<T> Handler for SimpleAuth<T>
where
    T: Handler + Clone + Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        let mut this = self.clone();

        // TODO -> the value should not be hardcoded
        let allowed = request.headers.contains_key("auth");

        Box::pin(async move {
            if !allowed {
                Ok(StatusCode::Unauthorized.into_response())
            } else {
                Ok(this.handler.call(request).await?)
            }
        })
    }
}
