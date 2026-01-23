// Credit: Ben Ajaero

use std::str;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Other(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
}

#[derive(Debug)]
pub enum ParseError {
    Empty,
    InvalidUtf8,
    InvalidRequestLine,
}

impl Request {
    pub fn parse(buffer: &[u8]) -> Result<Request, ParseError> {
        if buffer.is_empty() {
            return Err(ParseError::Empty);
        }

        let line_end = buffer
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or(ParseError::InvalidRequestLine)?;

        let line = str::from_utf8(&buffer[..line_end]).map_err(|_| ParseError::InvalidUtf8)?;
        let mut parts = line.split_whitespace();

        let method = parts.next().ok_or(ParseError::InvalidRequestLine)?;
        let path = parts.next().ok_or(ParseError::InvalidRequestLine)?;
        let version = parts.next().ok_or(ParseError::InvalidRequestLine)?;

        let method = match method {
            "GET" => Method::Get,
            other => Method::Other(other.to_string()),
        };

        Ok(Request {
            method,
            path: path.to_string(),
            version: version.to_string(),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
}

impl StatusCode {
    pub fn as_str(self) -> &'static str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::MethodNotAllowed => "405 Method Not Allowed",
            StatusCode::InternalServerError => "500 Internal Server Error",
        }
    }
}

pub fn response_bytes(status: StatusCode, content_type: &str, body: &[u8]) -> Vec<u8> {
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
        status.as_str(),
        body.len(),
        content_type
    );

    let mut response = Vec::with_capacity(header.len() + body.len());
    response.extend_from_slice(header.as_bytes());
    response.extend_from_slice(body);
    response
}

pub fn content_type_for_path(path: &str) -> &'static str {
    let path = path.to_ascii_lowercase();
    if path.ends_with(".html") || path.ends_with(".htm") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".txt") {
        "text/plain; charset=utf-8"
    } else {
        "application/octet-stream"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_request_line() {
        let raw = b"GET /index.html HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::parse(raw).expect("parse request");

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/index.html");
        assert_eq!(request.version, "HTTP/1.1");
    }

    #[test]
    fn parse_non_get_method() {
        let raw = b"POST /submit HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::parse(raw).expect("parse request");

        assert_eq!(request.method, Method::Other("POST".to_string()));
    }
}
