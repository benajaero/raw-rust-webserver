# Raw

Raw is a Rust-native web server framework focused on ergonomic routing, middleware,
and extensibility while keeping the core small and explicit.

## Why Raw (Mohist Design)
Raw addresses a common infrastructure gap in Rust services: predictable latency
under load for multi-tenant workloads. Instead of pushing admission control into
external proxies, Raw makes capacity explicit and fair at the framework level.

**Mohist principles in practice:**
- Universal benefit through fair admission control.
- Frugality via minimal overhead primitives.
- Clarity by exposing explicit route policies.
- Non-aggression by shedding load early.

## Features
- Async HTTP server built on Tokio + Hyper
- Declarative routing with params and wildcards
- Middleware chain (request ID, logging, static files)
- Response helpers for JSON, HTML, and text
- Admission control with route policies and global limits
- CLI scaffolding for new projects

## Terminal Preview
![cargo test output](docs/assets/cargo-test.svg)

## Quickstart
```rust
use raw::{App, Text};

#[tokio::main]
async fn main() {
    let mut app = App::new();
    app.get("/", |_req| async { Text::new("Hello from Raw") });
    app.listen("127.0.0.1:3000").await.unwrap();
}
```

## Route Policies (Mohist Admission Control)
```rust
use raw::{App, RoutePolicy, Text};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    let policy = RoutePolicy {
        max_in_flight: Some(16),
        cost: 1,
    };

    app.get_with("/reports/:id", policy, |_req| async {
        Text::new("report")
    });

    app.listen("127.0.0.1:3000").await.unwrap();
}
```

## CLI
```bash
cargo run -p raw-cli -- new my-raw-app
```

## Examples
```bash
cargo run -p raw --example basic
```

## Workspace layout
- `crates/raw` - framework core
- `crates/raw-cli` - CLI tools
- `docs/` - documentation and engineering notes

## Documentation
- `spec.md`
- `codex.md`
- `docs/ARCHITECTURE.md`
- `docs/DEVELOPMENT.md`
- `docs/CONTRIBUTING.md`
- `docs/CONFIGURATION.md`
- `docs/CLI.md`

## License
Licensed under Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International.
