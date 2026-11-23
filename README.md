# Sherlock

A Rust-first rebuild of the Sherlock Scholes football product.

The legacy application remains in [Aloews/sherlock-scholes-](https://github.com/Aloews/sherlock-scholes-). This repository is a separate implementation; the legacy production application and its data are not modified by work here.

## Current implementation

The Rust workspace now contains:
- an Axum API with `GET /healthz`;
- a Leptos WebAssembly client;
- a pure Rust Alias game engine with two alternating teams, timed rounds, correct/skip actions, scoring, and match results;
- a pure Rust Football Dynasty career engine with squad/formation management, player development, transfers, a deterministic four-club league, finances, family succession, and a Leptos client;
- a bundled 2,145-card catalogue imported from the legacy repository's `sherlock_cards.csv`.

The Alias mode runs locally on one device. Football Dynasty is a separate local career mode: it creates a fictional club and 22-player squad, supports formation and lineup changes, training, transfer purchases/sales, a six-matchday league, deterministic match simulation, standings, player appearances/goals, club finances, family heirs and succession. Dynasty saves automatically to this browser's local storage. It is not cloud-synchronized and does not use Telegram authentication or a remote database. Online Alias rooms, multiplayer synchronization, and live Supabase card reads are not implemented. The full legacy Football Dynasty parity contract is not yet met; see docs/DYNASTY_STATUS.md. GitHub CI and browser runtime verification are release gates; this repository is not production-ready.

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
