use std::collections::HashMap;

pub trait IntoResponse {
    fn into_response(self) -> HttpResponse;
}

// TODO -> normally it should be rather be a tuple struct
#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    OK,
    Unauthorized,
    NotFound,
    // ...
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> HttpResponse {
        match self {
            StatusCode::OK => HttpResponse::new(200, "OK".to_string()),
            StatusCode::Unauthorized => HttpResponse::new(401, "Unauthorized".to_string()),
            StatusCode::NotFound => HttpResponse::new(404, "Not Found".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    // body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: String) -> Self {
        HttpResponse {
            status_code,
            status_text,
            headers: HashMap::new(),
        }
    }

    // TODO -> hardcoded
    pub fn with_header(&mut self) {
        self.headers.insert(
            "Content-Type".to_string(),
            "application/json; charset=utf-8\r\n".to_string(),
        );

        self.headers
            .insert("Content-Length".to_string(), "138\r\n".to_string());
    }

    pub fn with_body(&mut self) {}

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        // status line
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        response.extend(status_line.as_bytes());

        // headers
        let headers_line =
            self.headers
                .iter()
                .fold(String::new(), |mut result, (header, header_val)| {
                    result.push_str(&format!("{}: {}", header, header_val));
                    result
                });
        response.extend(headers_line.as_bytes());

        // TODO -> should incl: Date, Content-Length

        // CRLF (Carriage Return Line Feed) to separate headers from body.
        // It serves as a delimiter between the headers and body sections of the HTTP response.
        response.extend(b"\r\n");

        // TODO -> write body

        response
    }
}
