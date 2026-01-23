# Architecture

## High-level flow
1. Hyper accepts incoming HTTP requests on a Tokio runtime.
2. The router matches the request path + method to a handler.
3. Middleware wraps the handler for cross-cutting concerns.
4. The handler returns a `Response` which is converted to Hyper's response type.

## Core modules
- `app` - application entry point and server lifecycle
- `router` - route table, path matching, and params
- `request` - typed request wrapper and query parsing
- `response` - response helpers and header utilities
- `middleware` - middleware chain and helpers
- `middleware_builtin` - request ID and logging middleware
- `static_files` - static file middleware

## Routing
Routes support:
- static segments: `/about`
- parameters: `/users/:id`
- wildcards: `/assets/*path`

## Middleware
Middleware is modeled after Koa-style chaining, allowing pre/post logic around
handlers. The chain runs even when a route is not found, enabling global
middleware like static files or logging.

## Admission Control
Routes can opt into admission control via `RoutePolicy`. Policies define a
maximum number of in-flight requests and a cost per request. Global limits are
also supported via `RawConfig`.
