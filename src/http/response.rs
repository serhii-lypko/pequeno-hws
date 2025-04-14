use std::collections::HashMap;

#[derive(Debug)]
pub struct HTTPResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    // body: Option<Vec<u8>>,
}

impl HTTPResponse {
    pub fn new(status_code: u16, status_text: String) -> Self {
        HTTPResponse {
            status_code,
            status_text,
            headers: HashMap::new(),
        }
    }

    pub fn with_header(&mut self) {}

    pub fn with_body(&mut self) {}

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        // "HTTP/1.1 200 OK\r\n\r\n"
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        response.extend(status_line.as_bytes());

        // CRLF (Carriage Return Line Feed) to separate headers from body.
        // It serves as a delimiter between the headers and body sections of the HTTP response.
        response.extend(b"\r\n");

        response
    }
}
