// Credit: Ben Ajaero

use crate::response::{Response, StatusCode};

#[derive(Debug)]
pub enum RawError {
    BadRequest,
    NotFound,
    MethodNotAllowed,
    Internal(String),
}

impl RawError {
    pub fn into_response(self) -> Response {
        match self {
            RawError::BadRequest => Response::new(StatusCode::BAD_REQUEST, "Bad Request", "text/plain"),
            RawError::NotFound => Response::new(StatusCode::NOT_FOUND, "Not Found", "text/plain"),
            RawError::MethodNotAllowed => {
                Response::new(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed", "text/plain")
            }
            RawError::Internal(message) => {
                Response::new(StatusCode::INTERNAL_SERVER_ERROR, message, "text/plain")
            }
        }
    }
}
