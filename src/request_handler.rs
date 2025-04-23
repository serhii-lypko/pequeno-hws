use std::{collections::HashMap, pin::Pin};
use tokio::time::{Duration, timeout};

use crate::http::{
    HttpRequest, HttpResponse,
    response::{IntoResponse, ResponseTrait, StatusCode},
};

// TODO -> generic for request; error as associated type
pub trait Service
where
    Self::Response: ResponseTrait,
{
    type Response;

    type Future: Future<Output = Result<Self::Response, anyhow::Error>> + Send;

    fn call(&mut self, request: HttpRequest) -> Self::Future;
}

// TODO -> could be expressed with trait either
type Handler =
    fn(HttpRequest) -> Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

// A simple router that maps paths to handlers
struct Router {
    // routes: Vec<(String, Handler)>,
    routes: Vec<Handler>,
}

impl Service for Router {
    type Response = HttpResponse;
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        // let path = request.path;

        let handler = self.routes[0];

        // TODO -> how to process next?
        let res = handler(request);

        let status_code = StatusCode::OK;
        let resp = status_code.into_response();

        Box::pin(async move { Ok(resp) })
    }
}

// TODO -> router should replace that
#[derive(Clone)]
pub struct RequestHandler;

impl Service for RequestHandler {
    type Response = HttpResponse;
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        // TODO -> need to handle router somehow
        // TODO: -> how router will looks like?

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

// impl<T> Service for Timeout<T>
// where
//     T: Service + Clone + Send + 'static,
// {
//     type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

//     fn call(&mut self, request: HttpRequest) -> Self::Future {
//         let mut this = self.clone();

//         Box::pin(async move {
//             let result = timeout(this.duration, this.handler.call(request)).await??;

//             // TODO -> propper result pattern matching
//             Ok(result)
//         })
//     }
// }

#[derive(Clone)]
pub struct SimpleAuth<T> {
    handler: T,
}

impl<T> SimpleAuth<T> {
    pub fn new(handler: T) -> Self {
        SimpleAuth { handler }
    }
}

// impl<T> Service for SimpleAuth<T>
// where
//     T: Service + Clone + Send + 'static,
// {
//     type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, anyhow::Error>> + Send>>;

//     fn call(&mut self, request: HttpRequest) -> Self::Future {
//         let mut this = self.clone();

//         // TODO -> the value should not be hardcoded
//         let allowed = request.headers.contains_key("auth");

//         Box::pin(async move {
//             if !allowed {
//                 Ok(StatusCode::Unauthorized.into_response())
//             } else {
//                 Ok(this.handler.call(request).await?)
//             }
//         })
//     }
// }
