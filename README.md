# Sherlock

A Rust-first rebuild of the Sherlock Scholes football product.

The legacy application remains in [Aloews/sherlock-scholes-](https://github.com/Aloews/sherlock-scholes-). This repository is a separate implementation; the legacy production application and its data are not modified by work here.

## Current state

This repository currently contains the initial Rust workspace and an API health endpoint. It is **not** a feature-complete application and is not ready for production.

## Architecture direction

- Rust backend: Axum and Tokio.
- PostgreSQL data boundary: preserve the existing Supabase data model and access policies until each domain has a verified replacement.
- Web and Telegram client: Rust/WASM UI, selected after a feature and browser compatibility spike.
- Existing football data collectors, scheduled jobs, and operational integrations must be inventoried before migration; they are not considered migrated merely because the API crate exists.

## Development

Requires a current stable Rust toolchain.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo run -p sherlock-api
```

The API health endpoint is available at `GET /healthz` (default bind address `0.0.0.0:8080`).

## Migration contract

See [the migration scope](docs/MIGRATION_SCOPE.md). User-facing feature parity, authorization, data integrity, realtime gameplay, Telegram flows, and production rollback are release gates. A successful compile or health check alone is not product acceptance.
