// Credit: Ben Ajaero

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use crate::middleware::{middleware, Middleware, Next};
use crate::request::Request;

static REQUEST_ID: AtomicU64 = AtomicU64::new(1);

pub fn request_id() -> Middleware {
    middleware(|req: Request, next: Next| async move {
        let id = REQUEST_ID.fetch_add(1, Ordering::Relaxed).to_string();
        let response = next.run(req).await;
        response.with_header("x-request-id", &id)
    })
}

pub fn logger() -> Middleware {
    middleware(|req: Request, next: Next| async move {
        let method = req.method().clone();
        let path = req.path().to_string();
        let start = Instant::now();
        let response = next.run(req).await;
        let duration = start.elapsed();
        eprintln!("{} {} -> {} ({} ms)", method, path, response.status(), duration.as_millis());
        response
    })
}

#[cfg(test)]
mod tests {
    use super::request_id;
    use crate::middleware::Next;
    use crate::request::Request;
    use crate::response::{Response, StatusCode};
    use hyper::Body;
    use std::collections::HashMap;

    #[tokio::test]
    async fn request_id_adds_header() {
        let middleware = request_id();
        let handler = crate::middleware::handler(|_req| async move {
            Response::empty(StatusCode::OK)
        });
        let next = Next::new(std::sync::Arc::new(Vec::new()), handler);
        let request = Request::new(hyper::Request::new(Body::empty()), HashMap::new());
        let response = (middleware)(request, next).await;
        let inner = response.into_inner();
        assert!(inner.headers().get("x-request-id").is_some());
    }
}
