// Credit: Ben Ajaero

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::request::Request;
use crate::response::Response;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

pub type Handler = Arc<dyn Fn(Request) -> BoxFuture<Response> + Send + Sync>;
pub type Middleware = Arc<dyn Fn(Request, Next) -> BoxFuture<Response> + Send + Sync>;

#[derive(Clone)]
pub struct Next {
    index: usize,
    middleware: Arc<Vec<Middleware>>,
    handler: Handler,
}

impl Next {
    pub(crate) fn new(middleware: Arc<Vec<Middleware>>, handler: Handler) -> Self {
        Self {
            index: 0,
            middleware,
            handler,
        }
    }

    fn step(&self) -> Option<Middleware> {
        self.middleware.get(self.index).cloned()
    }

    pub fn run(mut self, request: Request) -> BoxFuture<Response> {
        if let Some(layer) = self.step() {
            self.index += 1;
            return (layer)(request, self);
        }
        (self.handler)(request)
    }
}

pub fn handler<F, Fut>(f: F) -> Handler
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    Arc::new(move |req| Box::pin(f(req)))
}

pub fn middleware<F, Fut>(f: F) -> Middleware
where
    F: Fn(Request, Next) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    Arc::new(move |req, next| Box::pin(f(req, next)))
}
