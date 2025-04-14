use std::collections::HashMap;

use crate::http::Method;

type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    // body: Option<Vec<u8>>,
}

impl HTTPRequest {
    pub fn new(method: Method, path: String, headers: Option<Headers>) -> Self {
        HTTPRequest {
            method,
            path,
            headers: headers.unwrap_or_default(),
        }
    }
}
