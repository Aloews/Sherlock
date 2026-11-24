# Sherlock migration scope

> Canonical product and engineering map: [`docs/MAP.md`](MAP.md). Planned **Economic Dynasty** domain specification: [`docs/ECONOMIC_DYNASTY.md`](ECONOMIC_DYNASTY.md). Keep this file's concise scope and priority summary current because the autonomous task planner reads this file directly.

## Source baseline

Inspected the legacy repository `Aloews/sherlock-scholes-` on 2026-11-23.

- Recursive Git tree: 1,269 entries; tree response was not truncated.
- Main client: React, Vite, TypeScript.
- Data and backend: existing Supabase PostgreSQL, PostgREST/RPC, Edge Functions, plus scheduled GitHub Actions and collector services.
- Football Dynasty: a separate Rust/Axum service already exists under `services/football-dynasty`.
- Legacy product map: `docs/MAP.md`.

This is a repository inventory, not proof that every listed feature is currently operational in production. Each feature requires implementation and runtime verification.

## User-facing domains to preserve

1. Home, navigation, identity, and account/session restoration.
2. Football Alias: create/join rooms, teams, categories/cards, timer, scoring, results, replay, invites, and multiplayer reconnect.
3. Player catalogue, collectible cards, profiles, portraits, career and season records.
4. Clubs, squads, competitions, fixtures, results, tables, and match details.
5. Ratings, statistics, player comparisons, and transfers.
6. Match forecasts, prediction history, express analytics, award candidates, and tournament forecasts.
7. Fantasy team building and scoring.
8. Football news, digest, clips, and source attribution.
9. Friends, presence, social features, voice/video and related room experiences.
10. Amateur football spaces and owner-isolated management.
11. Football Dynasty campaign and simulation integration.
12. Existing non-admin user tools and settings discovered during route/API audit.
13. **Economic Dynasty (planned):** source-linked financial history, institutional memory, current markets and macroeconomics, business simulation, family capital, and intergenerational succession. This is a distinct future domain, not a current implementation claim.

Admin and operational interfaces are deliberately a later migration phase, but their hidden routes, APIs, permissions, jobs, and data dependencies must be inventoried now so that no capability is accidentally lost.

## Economic Dynasty — scope and sequencing contract

Economic Dynasty is a planned product expansion defined in `docs/MAP.md` and `docs/ECONOMIC_DYNASTY.md`. Its core research question is how financial knowledge is preserved or lost across families and institutions, and how historical/current events transmit through markets, firms, households, and later generations.

The target layers are historical events; geography and production; technology; money, credit and markets; political power and institutions; distribution of gains/losses; public narratives; and generational experience. Its product surface is a source-linked historical timeline/knowledge graph, a small transparent macro/market dashboard, and eventually a reproducible business/family dynasty simulation.

**Priority boundary:** keep the current primary football product parity and documented Rust Football Dynasty acceptance work ahead of Economic Dynasty implementation unless the product owner explicitly reprioritizes them. The economic domain must not be reduced to a placeholder screen or a generic “stock market game”. When it is prioritized, start with ED-0 research/source contracts, then a bounded historical timeline, then a verified current-market dashboard, and only then the business/family simulation.

**Agent planning requirement:** the autonomous planner derives new work from this file. When creating a task for Economic Dynasty, include the phrase “Economic Dynasty” and link/read `docs/ECONOMIC_DYNASTY.md` so the repository-context selector includes the dedicated specification. Do not create a live feed, historical claim, causal edge, market recommendation, family finance connection, or simulation mechanic without the evidence, contracts, and acceptance gates described there.

## Non-negotiable compatibility rules

- Do not modify or deploy the legacy production application as part of this migration.
- Do not copy production secrets into this public repository.
- Do not run destructive migrations or rewrite production data.
- Preserve account identity, ownership boundaries, RLS semantics, game history, and user-created content.
- Keep real-world football data separate from fictional Football Dynasty data.
- Keep historical facts, current observations, forecasts, estimates, and fictional simulation state in distinct data contracts.
- Never represent a simulated value, mock, fixture, seed, or placeholder as a live or historical fact.
- Preserve provenance, source timestamps, units, currencies, and the distinction between missing, zero, season, match, and career values.
- For Economic Dynasty, chronology is not causation; correlation is not causation; a beneficiary is not automatically an orchestrator.
- For money, use explicit currency and decimal-safe precision; do not use binary floating-point as the source of truth for balances.
- Keep family/business campaigns owner-scoped and do not connect personal bank or brokerage accounts in the initial scope.

## Delivery gates

### Gate A — inventory and contracts
Map routes, screens, feature modules, API calls, RPCs, Edge Functions, tables, RLS policies, scheduled jobs, external providers, and Telegram entry points. Capture request/response contracts and current behavior with tests or reproducible traces.

### Gate B — Rust domain and API
Port domain rules behind typed Rust interfaces. Add contract tests against the legacy behavior. Keep database changes additive and backwards-compatible.

### Gate C — primary client
Implement the user-facing flows in Rust/WASM after a browser/Telegram compatibility spike. Verify responsive layouts, deep links, authentication, loading/error/empty states, and accessibility.

### Gate D — parity and resilience
Exercise full journeys: Alias host/guest lifecycle, reconnect and expiry, profiles and clubs, fixtures, forecasts, fantasy, digest, amateur ownership, and Dynasty persistence. Verify real DB/API behavior rather than mocks alone.

### Gate E — release readiness
Run format, lint, unit, integration, browser, security, migration, and load checks. Verify observability, backup/restore, rollback, and production configuration. Do not switch production until explicit release approval and all gates pass.

### Gate F — admin migration
After the primary user-facing product has passed its gates, migrate hidden admin/operations features with least-privilege authorization, audit logging, and destructive-action safeguards.

### Gate G — Economic Dynasty
This gate is opened only by explicit product prioritization after the current core release gates are stable. Follow the staged acceptance gates ED-0 through ED-5 in `docs/ECONOMIC_DYNASTY.md`: research foundation, source-backed historical timeline/graph, verified market dashboard, reproducible family/business simulation, financial-memory validation, and secure Sherlock integration. No stage is complete on documentation or generated demo data alone.

## Current implementation in this repository

- Rust workspace with Axum API, a pure Rust game-core crate, and a Leptos WebAssembly client.
- `GET /healthz` endpoint in the API crate.
- Local two-team Football Alias match flow: setup, category selection, timed rounds, correct/skip actions, score tracking, alternating teams, round summaries, and final result.
- Local Football Dynasty career flow: fictional 22-player squad, lineup/formation management, training, transfer market, club cash ledger, deterministic four-club league, six-matchday season simulation, standings, player match records, heirs and succession. Save data is stored in the browser only.
- Bundled 2,145-card starter catalogue transformed from the legacy repository's `sherlock_cards.csv` (names and categories only).
- Unit and integration tests cover scoring, skipping, timer expiry, team alternation, match completion, and bundled catalogue loading.

**Important limitations:** Alias remains local to one device. Dynasty is also local-only: there is no server-side campaign API, Telegram identity, multi-device synchronization, account ownership/RLS, shared save recovery or remote backup. Its match simulation and generated clubs/players are fictional and deterministic, not real-world football data. Substitutions, tactical player instructions, rival-club squad development, generated injuries/suspensions, contract negotiation/renewal, detailed NPC motives and the full living-world loop remain outside this slice. The complete legacy Dynasty acceptance contract is tracked in docs/DYNASTY_STATUS.md. Online rooms, multiplayer/realtime, server-authoritative Alias scoring, Telegram identity, Supabase card reads, and persistent Alias match history remain unimplemented. Economic Dynasty, historical research ingestion, and live market data are not implemented. No production feature parity is claimed.
