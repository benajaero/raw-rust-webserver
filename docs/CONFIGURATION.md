# Configuration

The server reads configuration from environment variables.

| Variable | Default | Description |
| --- | --- | --- |
| `RWS_BIND_ADDR` | `127.0.0.1:7878` | Socket address to bind the listener |
| `RWS_THREADS` | `4` | Number of worker threads |
| `RWS_DOC_ROOT` | `public` | Directory containing static files |

Example:
```bash
RWS_BIND_ADDR=0.0.0.0:8080 RWS_THREADS=8 RWS_DOC_ROOT=public cargo run
```
