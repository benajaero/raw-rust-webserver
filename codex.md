# Codex Execution Plan

## Purpose
This document captures the execution plan for implementing the Raw framework described in `spec.md`. It defines the build order, constraints, and validation steps.

## Build Order
1. Repository rename and crate renaming
   - Rename repo to `raw-rust-web-server`.
   - Rename crate(s) to `raw` and align `Cargo.toml` metadata.
2. Core runtime
   - Introduce async runtime (Tokio) and HTTP server skeleton.
   - Implement `App` and request/response primitives.
3. Routing and middleware
   - Route table, path params, method routing.
   - Middleware chain with async handlers.
4. Static files and helpers
   - Static file middleware with cache headers.
   - Response helpers for JSON, HTML, text.
5. CLI tooling
   - Add `raw-cli` crate with `new`, `run`, `routes` commands.
6. Observability and config
   - Structured logging and request IDs.
   - Config loader with env overrides.
7. Documentation and examples
   - Guides, API reference, and example apps.

## Constraints
- Rust-only implementation.
- Keep API ergonomic but explicit.
- Prefer async/await and strong typing.
- Avoid hidden global state.

## Validation
- `cargo fmt`, `cargo clippy`, `cargo test` on each milestone.
- Unit tests for routing, middleware, and request parsing.
- Integration tests for static files and JSON responses.

## Deliverables
- `raw` crate (framework core).
- `raw-cli` crate (scaffolding and dev tools).
- Updated docs in `docs/` plus API reference.
