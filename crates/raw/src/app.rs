// Credit: Ben Ajaero

use std::convert::Infallible;
use std::net::TcpListener;
use std::sync::Arc;

use http::Method;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request as HyperRequest, Response as HyperResponse, Server};

use crate::error::RawError;
use crate::middleware::{handler, middleware, Middleware, Next};
use crate::request::Request;
use crate::response::Response;
use crate::router::Router;

pub struct App {
    router: Router,
    middleware: Vec<Middleware>,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            middleware: Vec::new(),
        }
    }

    pub fn get<F, Fut>(&mut self, path: &str, handler_fn: F)
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Response> + Send + 'static,
    {
        self.route(Method::GET, path, handler_fn);
    }

    pub fn post<F, Fut>(&mut self, path: &str, handler_fn: F)
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Response> + Send + 'static,
    {
        self.route(Method::POST, path, handler_fn);
    }

    pub fn route<F, Fut>(&mut self, method: Method, path: &str, handler_fn: F)
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Response> + Send + 'static,
    {
        let wrapped = handler(handler_fn);
        self.router.add(method, path, wrapped);
    }

    pub fn add_middleware<F, Fut>(&mut self, middleware_fn: F)
    where
        F: Fn(Request, Next) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Response> + Send + 'static,
    {
        self.middleware.push(middleware(middleware_fn));
    }

    pub async fn listen(self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).map_err(|err| {
            eprintln!("Failed to bind {}: {}", addr, err);
            err
        })?;
        self.serve(listener).await
    }

    pub async fn serve(
        self,
        listener: TcpListener,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        listener
            .set_nonblocking(true)
            .map_err(|err| {
                eprintln!("Failed to set non-blocking: {}", err);
                err
            })?;

        let state = Arc::new(self);
        let make_svc = make_service_fn(move |_| {
            let state = Arc::clone(&state);
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let state = Arc::clone(&state);
                    async move { state.handle(req).await }
                }))
            }
        });

        Ok(Server::from_tcp(listener)?.serve(make_svc).await?)
    }

    async fn handle(self: Arc<Self>, req: HyperRequest<Body>) -> Result<HyperResponse<Body>, Infallible> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        let response = if let Some(route_match) = self.router.find(&method, &path) {
            let request = Request::new(req, route_match.params);
            let handler = route_match.handler;
            let middleware = Arc::new(self.middleware.clone());
            let next = Next::new(middleware, handler);
            next.run(request).await
        } else if self.router.allows_path(&path) {
            RawError::MethodNotAllowed.into_response()
        } else {
            RawError::NotFound.into_response()
        };

        Ok(response.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::App;
    use crate::response::{Response, Text};

    #[tokio::test]
    async fn app_registers_route() {
        let mut app = App::new();
        app.get("/", |_req| async { Response::from(Text::new("ok")) });
        assert!(app.router.find(&http::Method::GET, "/").is_some());
    }
}
