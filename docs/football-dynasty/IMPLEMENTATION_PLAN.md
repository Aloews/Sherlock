# Football Dynasty — Implementation and Release Plan

## Imported implementation

- Rust/Axum API, SQLx persistence, Telegram identity-verification boundary, health endpoints, graceful shutdown and structured logging.
- PostgreSQL migrations for campaign, dynasty, characters, family, club ownership, ledgers, events, seasons, fixtures, match rosters/events, spatial match events and living-world reactions.
- Founder → heir → age boundary → succession → next-generation lifecycle.
- Bounded campaign calendar and persisted weekly decision inbox with idempotent choices.
- Fictional four-team league season, deterministic results, derived standings, match reports, goal/substitution events and player-match records.
- Living-world incident chain with location, actor knowledge, witnesses, persisted facts, delayed second-order reactions and idempotency.
- PostgreSQL-backed integration tests.

## Not yet verified or integrated in the new app

- CI on the new repository's exact branch head.
- Leptos screen/navigation and API wiring in the new public client.
- A dedicated database, new-service Telegram verifier and runtime credentials.
- API hosting, routing and authenticated browser/Telegram smoke tests.
- Backups, production telemetry and recovery checks.
- Full game-design target: multiple countries/competitions, deeper contracts/transfers, extended family legal system, politics, multi-club holdings and long-running autonomous NPC simulation.

## Release gates

1. Green formatter, Clippy, unit, integration and PostgreSQL migration tests on the exact commit.
2. Verify RLS and cross-owner denial tests.
3. Provision a separate database. Do not reuse legacy production DB, secrets or bot token.
4. Provision the new Telegram identity verifier and restricted runtime role.
5. Deploy the API separately from the static frontend; verify health/readiness.
6. Wire the Leptos screen to the API and complete authenticated end-to-end tests.
7. Confirm save/reload, idempotent commands, fixture reconciliation, ledger balance, event causality, succession and recovery before calling the mode production-ready.

A source-code import is not equivalent to a game accessible from the public UI.
