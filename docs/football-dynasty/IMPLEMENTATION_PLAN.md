# Football Dynasty — Implementation and Release Plan

## Imported backend

- Rust/Axum API, SQLx persistence, Telegram identity-verification boundary, health endpoints, graceful shutdown and structured logging.
- PostgreSQL migrations for campaign, dynasty, characters, family, club ownership, ledgers, events, seasons, fixtures, match rosters/events, spatial match events and living-world reactions.
- Founder → heir → age boundary → succession → next-generation lifecycle.
- Bounded campaign calendar and persisted weekly decision inbox with idempotent choices.
- Fictional four-team league season, deterministic results, derived standings, match reports, goal/substitution events and player-match records.
- Living-world incident chain with location, actor knowledge, witnesses, persisted facts, delayed second-order reactions and idempotency.
- PostgreSQL-backed integration coverage for campaign lifecycle, owner isolation, event resolution, seasons, match events, ledgers and succession.

## Leptos client added in the new repository

- A mode tab opens Football Dynasty alongside Football Alias.
- Campaign list and campaign creation.
- Dynasty/founder/club overview and family roster.
- Add an heir, advance the calendar, resolve event choices.
- Show cash accounts, chronicle, league standings and season fixtures.
- Reads Telegram Mini App `initData` from the Telegram WebApp bridge and sends it to the backend; the browser never supplies an owner ID.
- Uses `SHERLOCK_DYNASTY_API_URL` at compile time, defaulting to same-origin API paths.

## Not yet production-ready

- A dedicated database, new-service Telegram verifier and runtime credentials.
- API hosting, routing, production CORS origin and authenticated browser/Telegram smoke tests.
- Backup/restore, production telemetry and recovery checks.
- Full game-design target: multiple countries/competitions, deeper contracts/transfers, extended family legal system, politics, multi-club holdings and long-running autonomous NPC simulation.

## Release gates

1. Green formatter, Clippy, Rust/WASM, browser-bundle, PostgreSQL integration and release builds on the exact commit.
2. Verify RLS and cross-owner denial tests.
3. Provision a separate database. Do not reuse legacy production DB, secrets or bot token.
4. Provision the new Telegram identity verifier and restricted runtime role.
5. Deploy the API separately from the static frontend; verify health/readiness and exact CORS.
6. Merge/deploy the Leptos client only when API configuration is ready.
7. Confirm authenticated create/save/reload, idempotent commands, calendar, event choices, season/match report, ledger balance, event causality and succession before calling the mode production-ready.

A successful code build or disposable PostgreSQL test does not prove the public production game is deployed.
