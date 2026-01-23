// Credit: Ben Ajaero

use hyper::{Body, Response as HyperResponse};

#[derive(Debug)]
pub struct Response {
    inner: HyperResponse<Body>,
}

impl Response {
    pub fn new(status: StatusCode, body: impl Into<Body>, content_type: &str) -> Self {
        let mut response = HyperResponse::new(body.into());
        *response.status_mut() = status;
        response.headers_mut().insert(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_str(content_type).unwrap_or_else(|_| {
                http::HeaderValue::from_static("application/octet-stream")
            }),
        );
        Self { inner: response }
    }

    pub fn empty(status: StatusCode) -> Self {
        let mut response = HyperResponse::new(Body::empty());
        *response.status_mut() = status;
        Self { inner: response }
    }

    pub fn into_inner(self) -> HyperResponse<Body> {
        self.inner
    }
}

impl From<Response> for HyperResponse<Body> {
    fn from(response: Response) -> Self {
        response.inner
    }
}

pub struct Text {
    body: String,
}

impl Text {
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }
}

impl From<Text> for Response {
    fn from(text: Text) -> Self {
        Response::new(StatusCode::OK, text.body, "text/plain; charset=utf-8")
    }
}

pub struct Html {
    body: String,
}

impl Html {
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }
}

impl From<Html> for Response {
    fn from(html: Html) -> Self {
        Response::new(StatusCode::OK, html.body, "text/html; charset=utf-8")
    }
}

pub struct Json {
    body: serde_json::Value,
}

impl Json {
    pub fn new(body: serde_json::Value) -> Self {
        Self { body }
    }
}

impl From<Json> for Response {
    fn from(json: Json) -> Self {
        let payload = serde_json::to_string(&json.body).unwrap_or_else(|_| "{}".to_string());
        Response::new(StatusCode::OK, payload, "application/json")
    }
}

pub use http::StatusCode;

#[cfg(test)]
mod tests {
    use super::{Html, Json, Response, StatusCode, Text};

    #[test]
    fn text_response_sets_content_type() {
        let response: Response = Text::new("hello").into();
        let inner = response.into_inner();
        assert_eq!(inner.status(), StatusCode::OK);
        assert_eq!(
            inner.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/plain; charset=utf-8"
        );
    }

    #[test]
    fn html_response_sets_content_type() {
        let response: Response = Html::new("<h1>Raw</h1>").into();
        let inner = response.into_inner();
        assert_eq!(
            inner.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[test]
    fn json_response_sets_content_type() {
        let response: Response = Json::new(serde_json::json!({"ok": true})).into();
        let inner = response.into_inner();
        assert_eq!(inner.status(), StatusCode::OK);
        assert_eq!(
            inner.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }
}
