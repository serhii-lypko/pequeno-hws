use std::convert::TryFrom;

pub use request::HTTPRequest;
pub use response::HTTPResponse;

pub mod request;
pub mod response;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl TryFrom<&str> for Method {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(anyhow::anyhow!("Unsupported Method")),
        }
    }
}
