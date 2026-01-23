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
The current request parser is minimal and looks for a `GET /` prefix in the
byte buffer. This is intentionally simple to keep focus on concurrency basics.
