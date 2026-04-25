# Changelog

All notable changes to this project will be documented in this file.

## Unreleased
- Introduce the Raw framework core with async routing and middleware.
- Add static file middleware, request ID, and logging helpers.
- Provide request helpers for JSON parsing and response helpers for JSON/HTML/text.
- Add the Raw CLI with project scaffolding.
- Expand docs and examples for the new framework.
- Add Mohist admission control with per-route policies and global concurrency limits.
- Allow handlers to return `Into<Response>` for ergonomic usage.
- Add terminal test preview graphic to README.

## Known Gaps
- CLI `routes` and `run` commands are stubbed (not yet implemented).
- No graceful shutdown wired in `serve()`.
- No route groups with scoped middleware.
- No `raw-macros` crate.
- No benchmark suite.
- No configuration file loading (YAML/TOML).
- No template / SSR support.
- No observability hooks (metrics, tracing).
- HTTP/2 support pending.
