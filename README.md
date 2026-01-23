# raw-rust-webserver

A minimal, multi-threaded HTTP server built with the Rust standard library.
This project is intentionally small and dependency-free to make the core ideas
of networking, request handling, and thread pools easy to study.

## Features
- Fixed-size thread pool
- Handles simple HTTP GET requests
- Serves a static HTML response for `/`
- Returns a 404 page for unknown routes

## Quickstart
```bash
cargo run
```
Then open `http://127.0.0.1:7878` in a browser.

## Project layout
- `src/bin/main.rs` - server entrypoint and connection handling
- `src/lib.rs` - thread pool implementation
- `hello.html` - response body for `/`
- `404.html` - response body for unknown routes
- `docs/` - engineering notes and contribution guidelines

## Documentation
- `docs/ARCHITECTURE.md`
- `docs/DEVELOPMENT.md`
- `docs/CONTRIBUTING.md`

## Roadmap
This repo is used for iterative improvements. Planned areas include:
- better HTTP parsing and headers
- configurable bind address and thread count
- structured logging and error handling
- static file routing

## License
This project is currently unlicensed. Add a license before production use.
