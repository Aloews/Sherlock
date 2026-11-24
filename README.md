# Sherlock

A Rust-first rebuild of the Sherlock Scholes product, with a planned evolution into a persistent whole-world simulation.

The legacy application remains in [Aloews/sherlock-scholes-](https://github.com/Aloews/sherlock-scholes-). This repository is a separate implementation; the legacy production application and its data are not modified by work here.

## Product direction

Sherlock's long-term world is broader than football. It is designed to model interacting geography, environment, population, culture, knowledge, technology, production, trade, finance, institutions, politics and individual lives across time and generations. Football is one specialized domain within that world—not the world's organizing boundary. Football clubs, players, competitions, supporters and real football reference data should connect to the wider economy and society while remaining a clearly bounded domain.

The whole-world architecture and phased implementation proposal are documented in [Historical World Simulation](docs/HISTORICAL_WORLD_SIMULATION.md). This is a proposed specification, not a claim that the world simulation is implemented. The current playable scope remains limited as described below.

The legacy application remains the source baseline for football-product migration. Real-world football facts, fictional Football Dynasty campaign state, and any future historical/simulated world state must remain distinguishable and carry appropriate provenance.

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
