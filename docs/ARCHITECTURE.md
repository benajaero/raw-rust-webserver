# Architecture

## High-level flow
1. `TcpListener` accepts incoming connections.
2. Each connection is handed to the `ThreadPool` for concurrent handling.
3. The connection handler reads a single request and writes a response.

## Thread pool
The thread pool owns a fixed set of worker threads. Jobs are sent to workers
over a channel. When the pool is dropped, each worker receives a termination
message and joins cleanly.

## Request handling
The request parser reads the request line and extracts the method, path, and
HTTP version. Only `GET` is handled today.

## Routing and static files
- `/health` returns a plain-text `ok` response.
- Other paths resolve to files under the configured document root.
- `index.html` is served when the path is `/`.
