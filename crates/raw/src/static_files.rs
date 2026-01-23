// Credit: Ben Ajaero

use std::path::{Path, PathBuf};

use http::Method;

use crate::middleware::{middleware, Middleware, Next};
use crate::request::Request;
use crate::response::{Response, StatusCode};

pub fn static_files(root: impl Into<PathBuf>) -> Middleware {
    let root = root.into();
    middleware(move |req: Request, next: Next| {
        let root = root.clone();
        async move {
            if req.method() != Method::GET {
                return next.run(req).await;
            }

            let path = match sanitize_path(req.path()) {
                Some(path) => path,
                None => {
                    return Response::new(StatusCode::BAD_REQUEST, "Bad Request", "text/plain");
                }
            };

            let candidate = root.join(path);
            match tokio::fs::read(&candidate).await {
                Ok(contents) => Response::new(StatusCode::OK, contents, content_type_for_path(&candidate)),
                Err(_) => next.run(req).await,
            }
        }
    })
}

fn sanitize_path(path: &str) -> Option<PathBuf> {
    let trimmed = path.trim_start_matches('/');
    if trimmed.contains("..") {
        return None;
    }

    let normalized = if trimmed.is_empty() {
        PathBuf::from("index.html")
    } else {
        PathBuf::from(trimmed)
    };

    Some(normalized)
}

fn content_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::{content_type_for_path, sanitize_path};
    use std::path::Path;

    #[test]
    fn sanitize_rejects_parent() {
        assert!(sanitize_path("/../secret").is_none());
    }

    #[test]
    fn sanitize_defaults_index() {
        let path = sanitize_path("/").expect("path");
        assert_eq!(path.to_string_lossy(), "index.html");
    }

    #[test]
    fn content_type_defaults_binary() {
        assert_eq!(content_type_for_path(Path::new("/tmp/file.bin")), "application/octet-stream");
    }
}
