# Development

## Requirements
- Rust toolchain (stable)

## Build
```bash
cargo build
```

## Run
```bash
cargo run
```

## Configure
```bash
RWS_BIND_ADDR=127.0.0.1:7878 RWS_THREADS=4 RWS_DOC_ROOT=public cargo run
```

## Format
```bash
cargo fmt
```

## Lint
```bash
cargo clippy
```
