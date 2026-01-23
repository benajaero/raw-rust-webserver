// Credit: Ben Ajaero

use std::collections::HashMap;

use http::Method;
use hyper::{Body, Request as HyperRequest};

#[derive(Debug)]
pub struct Request {
    inner: HyperRequest<Body>,
    params: HashMap<String, String>,
    query: HashMap<String, String>,
}

impl Request {
    pub(crate) fn new(inner: HyperRequest<Body>, params: HashMap<String, String>) -> Self {
        let query = parse_query(inner.uri().query());
        Self {
            inner,
            params,
            query,
        }
    }

    pub fn method(&self) -> &Method {
        self.inner.method()
    }

    pub fn path(&self) -> &str {
        self.inner.uri().path()
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(String::as_str)
    }

    pub fn query(&self, key: &str) -> Option<&str> {
        self.query.get(key).map(String::as_str)
    }

    pub fn headers(&self) -> &http::HeaderMap {
        self.inner.headers()
    }

    pub fn into_body(self) -> Body {
        self.inner.into_body()
    }
}

fn parse_query(query: Option<&str>) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let Some(query) = query else {
        return params;
    };

    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut iter = pair.splitn(2, '=');
        let key = iter.next().unwrap_or_default();
        let value = iter.next().unwrap_or("");
        params.insert(key.to_string(), value.to_string());
    }

    params
}

#[cfg(test)]
mod tests {
    use super::parse_query;

    #[test]
    fn parse_query_pairs() {
        let parsed = parse_query(Some("name=raw&mode=fast"));
        assert_eq!(parsed.get("name"), Some(&"raw".to_string()));
        assert_eq!(parsed.get("mode"), Some(&"fast".to_string()));
    }
}
