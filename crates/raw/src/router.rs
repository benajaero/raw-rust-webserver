// Credit: Ben Ajaero

use std::collections::HashMap;

use http::Method;

use crate::middleware::Handler;

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add(&mut self, method: Method, path: &str, handler: Handler) {
        let pattern = PathPattern::new(path);
        self.routes.push(Route {
            method,
            pattern,
            handler,
        });
    }

    pub fn find(&self, method: &Method, path: &str) -> Option<RouteMatch> {
        for route in &self.routes {
            if &route.method != method {
                continue;
            }
            if let Some(params) = route.pattern.match_path(path) {
                return Some(RouteMatch {
                    handler: route.handler.clone(),
                    params,
                });
            }
        }
        None
    }

    pub fn allows_path(&self, path: &str) -> bool {
        self.routes
            .iter()
            .any(|route| route.pattern.match_path(path).is_some())
    }
}

struct Route {
    method: Method,
    pattern: PathPattern,
    handler: Handler,
}

pub struct RouteMatch {
    pub handler: Handler,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct PathPattern {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
enum Segment {
    Static(String),
    Param(String),
    Wildcard(String),
}

impl PathPattern {
    fn new(path: &str) -> Self {
        let segments = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(|segment| {
                if let Some(name) = segment.strip_prefix(':') {
                    Segment::Param(name.to_string())
                } else if let Some(name) = segment.strip_prefix('*') {
                    Segment::Wildcard(name.to_string())
                } else {
                    Segment::Static(segment.to_string())
                }
            })
            .collect();
        Self { segments }
    }

    fn match_path(&self, path: &str) -> Option<HashMap<String, String>> {
        let parts: Vec<&str> = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();

        if self.segments.is_empty() && parts.is_empty() {
            return Some(HashMap::new());
        }

        let mut params = HashMap::new();
        let mut index = 0;
        for segment in &self.segments {
            match segment {
                Segment::Static(expected) => {
                    let Some(actual) = parts.get(index) else {
                        return None;
                    };
                    if expected != actual {
                        return None;
                    }
                    index += 1;
                }
                Segment::Param(name) => {
                    let Some(actual) = parts.get(index) else {
                        return None;
                    };
                    params.insert(name.clone(), (*actual).to_string());
                    index += 1;
                }
                Segment::Wildcard(name) => {
                    let rest = parts.get(index..).unwrap_or_default().join("/");
                    params.insert(name.clone(), rest);
                    return Some(params);
                }
            }
        }

        if index == parts.len() {
            Some(params)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PathPattern;

    #[test]
    fn match_static_path() {
        let pattern = PathPattern::new("/users");
        assert!(pattern.match_path("/users").is_some());
        assert!(pattern.match_path("/users/1").is_none());
    }

    #[test]
    fn match_param_path() {
        let pattern = PathPattern::new("/users/:id");
        let params = pattern.match_path("/users/42").expect("match");
        assert_eq!(params.get("id"), Some(&"42".to_string()));
    }

    #[test]
    fn match_wildcard_path() {
        let pattern = PathPattern::new("/assets/*path");
        let params = pattern.match_path("/assets/css/main.css").expect("match");
        assert_eq!(params.get("path"), Some(&"css/main.css".to_string()));
    }
}
