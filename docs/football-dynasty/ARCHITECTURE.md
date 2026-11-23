# Football Dynasty — Architecture

## Runtime boundary

Sherlock's public client is a Rust + Leptos/WebAssembly application. Football Dynasty is a separate Axum/Tokio/SQLx service under `services/football-dynasty`. The API owns authoritative game rules, campaign identity enforcement, idempotency, simulation and database transactions.

The service is deployed independently from the static WebAssembly frontend and uses a dedicated PostgreSQL database. It must not connect to the older Sherlock Scholes production database or mutate legacy real-football and Alias tables.

## Identity boundary

The imported API accepts Telegram Mini App `initData` in `x-tg-init-data`. It obtains the owner identity only through its server-side verifier, currently backed by `public.tg_validate_init_data(text)`. A browser-supplied Telegram ID is never trusted.

Before production, provision a new database and a production-grade verifier backed by a bot token stored only in the new service's secret store. The bundled database integration test creates a deliberately rejecting stub function; it is test-only and must never be deployed.

## Data boundary

Fictional clubs, players, family members, competitions, fixtures, assets, accounts, journal entries, events and chronicle records live in the private `fd` schema. Every campaign-owned relation is scoped to its campaign and authenticated owner through RLS and the owner set by the API transaction.

## Command boundary

All mutations are API commands, not browser-side SQL. Mutations require idempotency keys. The backend validates preconditions and commits related writes atomically. The browser must not submit balances, legal ownership, heir eligibility, campaign clock or simulation output as authoritative state.

## Simulation

Campaign time is paused between player commands and advances in bounded day-level batches. Seeds and persistent state make outcomes reproducible and resumable. A campaign's game calendar—not server wall time—is the source of truth.

## Deployment gates

- New dedicated PostgreSQL database and restricted runtime credentials.
- New Telegram identity verifier and secret, independent of the old application.
- Migrations applied only to the new database.
- API deployed independently, with health/readiness verified.
- Leptos screen connected to the API.
- Authenticated create/save/reload, season, event and succession tests pass.
