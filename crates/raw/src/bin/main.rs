// Credit: Ben Ajaero

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use raw::config::Config;
use raw::http::{content_type_for_path, response_bytes, Method, Request, StatusCode};
use raw::ThreadPool;

fn main() {
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Configuration error: {}", err);
            std::process::exit(1);
        }
    };

    let listener = TcpListener::bind(&config.bind_addr).unwrap_or_else(|err| {
        eprintln!("Failed to bind {}: {}", config.bind_addr, err);
        std::process::exit(1);
    });
    let pool = ThreadPool::new(config.threads);
    let config = Arc::new(config);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let config = Arc::clone(&config);
                pool.execute(|| {
                    handle_connection(stream, config);
                });
            }
            Err(err) => eprintln!("Connection failed: {}", err),
        }

    }
}

fn handle_connection(mut stream: TcpStream, config: Arc<Config>) {
    let mut buffer = [0; 4096];

    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Failed to read request: {}", err);
            return;
        }
    };

    let request = match Request::parse(&buffer[..bytes_read]) {
        Ok(request) => request,
        Err(_) => {
            let response = response_bytes(
                StatusCode::BadRequest,
                "text/plain; charset=utf-8",
                b"Bad Request",
            );
            let _ = stream.write_all(&response);
            let _ = stream.flush();
            return;
        }
    };

    if request.path == "/health" {
        let response = response_bytes(StatusCode::Ok, "text/plain; charset=utf-8", b"ok");
        let _ = stream.write_all(&response);
        let _ = stream.flush();
        return;
    }

    let (status, body, content_type) = match request.method {
        Method::Get => match resolve_path(&config.doc_root, &request.path) {
            Some(path) => match fs::read(&path) {
                Ok(contents) => {
                    let content_type = content_type_for_path(path.to_string_lossy().as_ref());
                    (StatusCode::Ok, contents, content_type)
                }
                Err(_) => (
                    StatusCode::NotFound,
                    load_fallback(&config.doc_root),
                    "text/html; charset=utf-8",
                ),
            },
            None => (
                StatusCode::BadRequest,
                b"Bad Request".to_vec(),
                "text/plain; charset=utf-8",
            ),
        },
        Method::Other(_) => (
            StatusCode::MethodNotAllowed,
            b"Method Not Allowed".to_vec(),
            "text/plain; charset=utf-8",
        ),
    };

    let response = response_bytes(status, content_type, &body);
    let _ = stream.write_all(&response);
    let _ = stream.flush();
}

fn resolve_path(doc_root: &Path, request_path: &str) -> Option<PathBuf> {
    let path = request_path.split('?').next().unwrap_or("/");
    let trimmed = path.trim_start_matches('/');

    let relative = if trimmed.is_empty() {
        "index.html"
    } else {
        trimmed
    };

    if relative.contains("..") {
        return None;
    }

    Some(doc_root.join(relative))
}

fn load_fallback(doc_root: &Path) -> Vec<u8> {
    let fallback_path = doc_root.join("404.html");
    fs::read(&fallback_path).unwrap_or_else(|_| b"Not Found".to_vec())
}
