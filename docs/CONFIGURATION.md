# Configuration

The framework exposes a `RawConfig` type that can be constructed from
environment variables.

| Variable | Default | Description |
| --- | --- | --- |
| `RAW_BIND_ADDR` | `127.0.0.1:3000` | Socket address to bind the listener |
| `RAW_WORKERS` | `4` | Worker thread count for the Tokio runtime |

Example:
```bash
RAW_BIND_ADDR=0.0.0.0:8080 RAW_WORKERS=8 cargo run
```
