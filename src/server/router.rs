use crate::http::{HTTPRequest, HTTPResponse};

type BoxedHandler = Box<dyn Fn(HTTPRequest) -> HTTPResponse + Send + Sync>;

// TODO -> trie based pathfinding (prefix-trie)
pub struct Router {
    pub routes: Vec<Route>,
}

pub struct Route {
    pub path: String,
    pub handler: BoxedHandler,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn route(mut self, handler: BoxedHandler) -> Self {
        let route = Route {
            path: "/".to_string(),
            handler,
        };

        self.routes.push(route);

        self
    }
}
