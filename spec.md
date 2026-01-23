# Raw Framework Specification

## Overview
Raw is a Rust-native web server framework focused on ergonomic routing, middleware, and extensibility while retaining strong performance and explicit control. The framework targets building HTTP services and web applications with minimal boilerplate and a clear, composable API.

## Goals
- Provide a production-ready HTTP framework with a clean, opinionated core.
- Offer fast request routing, middleware composition, and structured responses.
- Support static assets, JSON APIs, and server-rendered HTML.
- Provide solid observability (logging, metrics hooks, tracing integration).
- Deliver an ergonomic developer experience with a CLI and scaffolding.

## Non-Goals
- No browser UI framework or client-side runtime (this is server-side only).
- No runtime hot-reload in v1 (can be a later add-on).
- No ORM bundled by default (integrations via examples or plugins).

## Target Audience
- Rust developers building APIs, microservices, or server-rendered web apps.
- Teams wanting a lightweight, explicit framework without hidden magic.

## Core Features (v1)
### HTTP Server
- Async HTTP server with graceful shutdown.
- Configurable bind address, worker threads, and timeouts.
- Support for HTTP/1.1; HTTP/2 optional if dependency supports it.

### Routing
- Declarative routing with path params, wildcards, and query access.
- Method-based routing with a single route table.
- Route groups with scoped middleware.

### Request/Response
- Strongly typed `Request` and `Response` types.
- Helpers for JSON, HTML, text, and file responses.
- Streaming response support for large payloads.

### Middleware
- Koa-style middleware chain using async handlers.
- Built-in middleware: logging, request ID, CORS, compression, static files.

### Error Handling
- Unified error type with HTTP status mapping.
- Custom error handlers per app or per route.

### Static Files
- Efficient static file serving with caching headers.
- Configurable document root and index file.

### Templates / SSR
- Optional server-side templating support via a feature flag.
- Render helpers for HTML responses.

### CLI
- `raw new` scaffolds a new project layout.
- `raw run` runs the development server.
- `raw routes` prints registered routes.

### Observability
- Structured logging with JSON output option.
- Integration hooks for metrics and tracing.

### Configuration
- `RawConfig` with environment variable overrides.
- YAML/TOML config loading optional (feature flag).

## Architecture
### Crates
- `raw` (main framework crate)
- `raw-cli` (CLI tooling)
- `raw-macros` (optional proc macros for routes)

### Modules
- `server` - listener, runtime, graceful shutdown
- `router` - route table, matchers
- `request` - request parsing, body extraction
- `response` - response builders, body types
- `middleware` - middleware traits and chain
- `errors` - unified error types
- `static_files` - file serving
- `templates` - optional templates
- `config` - settings and env loading

## API Sketch
```rust
use raw::{App, Router, response::Html};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/", |_req| async {
        Html::new("<h1>Hello from Raw</h1>")
    });

    app.get("/users/:id", |req| async move {
        let id = req.param("id");
        raw::response::Json::new(serde_json::json!({"id": id}))
    });

    app.listen("127.0.0.1:3000").await.unwrap();
}
```

## Milestones
1. Core server runtime and request/response primitives.
2. Router and middleware chain.
3. Static files and JSON helpers.
4. CLI scaffolding and templates.
5. Observability and configuration polish.

## Quality Targets
- 90%+ test coverage on core routing and middleware.
- Benchmarks for request throughput and latency.
- Complete docs: getting started, guides, and API reference.
