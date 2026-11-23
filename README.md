# Sherlock

A Rust-first rebuild of the Sherlock Scholes football product.

The legacy application remains in [Aloews/sherlock-scholes-](https://github.com/Aloews/sherlock-scholes-). This repository is a separate implementation; the legacy production application and its data are not modified by work here.

## Current implementation

The Rust workspace now contains:
- an Axum API with `GET /healthz`;
- a Leptos WebAssembly client;
- a pure Rust Alias game engine with two alternating teams, timed rounds, correct/skip actions, scoring, and match results;
- a bundled 2,145-card catalogue imported from the legacy repository's `sherlock_cards.csv`;
- a separate pure Rust Football Dynasty domain engine and Leptos mode for local campaign creation, a fictional club league and season, deterministic match results, standings, event choices, double-entry finance, family/heir lifecycle, succession, chronicle, and local save/reload.

Open the modes from the top navigation, or directly at `/?mode=alias` and `/?mode=dynasty`. Dynasty campaigns are stored in this browser's local storage only. They are not cloud-synced or server-authoritative. The original Dynasty product's complete feature set is not yet ported: player-level match tactics, transfers and contracts, senior staff, ownership structures, politics, fully autonomous rival dynasties and the complete living-world reaction graph remain future work.

Online rooms, multiplayer synchronization, Telegram authentication, shared accounts, server persistence, and live Supabase reads are not implemented in this initial slice. GitHub CI and real browser runtime verification are release gates; this repository is not yet production-complete.

## Development

Requires a current stable Rust toolchain and the `wasm32-unknown-unknown` target.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo check -p sherlock-web --target wasm32-unknown-unknown
cargo run -p sherlock-api
```

To serve the WebAssembly app locally, install Trunk and run it from `crates/web`:

```sh
cd crates/web
trunk serve
```

## Migration contract

See [the migration scope](docs/MIGRATION_SCOPE.md). User-facing feature parity, authorization, data integrity, realtime gameplay, Telegram flows, and production rollback are release gates. A successful compile or health check alone is not product acceptance.
