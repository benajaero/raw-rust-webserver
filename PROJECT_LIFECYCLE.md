# Raw Framework — Weekly Lifecycle

A rotating schedule to keep the project moving without decision fatigue.

| Day       | Activity Type              | Focus Area                                  |
|-----------|---------------------------|---------------------------------------------|
| Monday    | Core Engineering          | Server runtime, routing, middleware, perf   |
| Tuesday   | Feature Development       | New handlers, response types, extensions    |
| Wednesday | Tooling & DX              | CLI commands, scaffolding, dev server       |
| Thursday  | Testing & Quality         | Unit tests, integration tests, coverage     |
| Friday    | Documentation & Examples    | Docs, README, guides, API sketches          |
| Saturday  | Integration & Polish      | CI, config, dependency bumps, packaging     |
| Sunday    | Review & Cleanup          | Spec alignment, debt reduction, planning    |

## Sunday Review Checklist

1. Read `spec.md` — check for drift vs. implementation
2. Scan for `TODO`, `FIXME`, `unimplemented`, `coming soon`
3. Check `CHANGELOG.md` is current
4. Ensure all `.rs` files have the credit header
5. Review open gaps between milestones and code
6. Plan next week's priority item

## Current Gaps (as of 2026-04-26)

- CLI `routes` and `run` commands are stubbed (not implemented)
- No benchmark suite
- No HTTP/2 support (spec mentions it as optional)
- Templates / SSR not started
- Observability hooks not started
- Configuration file loading not started
- `raw-macros` crate does not exist
- Route groups with scoped middleware not implemented
- `serve()` does not wire graceful shutdown (Hyper 0.14 supports it)
- `codex.md` referenced in README but removed from repo

## Milestone Status

1. ✅ Core server runtime and request/response primitives
2. ✅ Router and middleware chain
3. ✅ Static files and JSON helpers
4. 🟡 CLI scaffolding done; `routes` and `run` pending
5. ⬜ Observability and configuration polish
6. ✅ Mohist admission control (basic)

## Next Week Priority

Implement `raw run` dev server command in CLI.
