# raw-rust-webserver

A minimal, multi-threaded HTTP server built with the Rust standard library.
This project is intentionally small and dependency-free to make the core ideas
of networking, request handling, and thread pools easy to study.

## Features
- Fixed-size thread pool
- Minimal HTTP request parsing
- Serves static files from `public/`
- Health check at `/health`
- Environment-based configuration

## Quickstart
```bash
cargo run
```
Then open `http://127.0.0.1:7878` in a browser.

### Configuration
```bash
RWS_BIND_ADDR=127.0.0.1:7878
RWS_THREADS=4
RWS_DOC_ROOT=public
```

## Project layout
- `src/bin/main.rs` - server entrypoint and connection handling
- `src/lib.rs` - thread pool implementation
- `public/` - static files served by the server
- `docs/` - engineering notes and contribution guidelines

## Documentation
- `docs/ARCHITECTURE.md`
- `docs/DEVELOPMENT.md`
- `docs/CONTRIBUTING.md`
- `docs/CONFIGURATION.md`

## Roadmap
This repo is used for iterative improvements. Planned areas include:
- better HTTP parsing and headers
- configurable bind address and thread count
- structured logging and error handling
- static file routing

## License
This project is currently unlicensed. Add a license before production use.
