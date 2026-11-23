# Sherlock migration scope

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

Admin and operational interfaces are deliberately a later migration phase, but their hidden routes, APIs, permissions, jobs, and data dependencies must be inventoried now so that no capability is accidentally lost.

## Non-negotiable compatibility rules

- Do not modify or deploy the legacy production application as part of this migration.
- Do not copy production secrets into this public repository.
- Do not run destructive migrations or rewrite production data.
- Preserve account identity, ownership boundaries, RLS semantics, game history, and user-created content.
- Keep real-world football data separate from fictional Football Dynasty data.
- Preserve provenance, source timestamps, and the distinction between missing, zero, season, match, and career values.
- Never represent mocks, fixtures, or seed data as live production data.

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

## Current implementation in this repository

- Rust workspace with Axum API, a pure Rust game-core crate, and a Leptos WebAssembly client.
- `GET /healthz` endpoint in the API crate.
- Local two-team Football Alias match flow: setup, category selection, timed rounds, correct/skip actions, score tracking, alternating teams, round summaries, and final result.
- Local Football Dynasty career flow: fictional 22-player squad, lineup/formation management, training, transfer market, club cash ledger, deterministic four-club league, six-matchday season simulation, standings, player match records, heirs and succession. Save data is stored in the browser only.
- Bundled 2,145-card starter catalogue transformed from the legacy repository's `sherlock_cards.csv` (names and categories only).
- Unit and integration tests cover scoring, skipping, timer expiry, team alternation, match completion, and bundled catalogue loading.

**Important limitations:** Alias remains local to one device. Dynasty is also local-only: there is no server-side campaign API, Telegram identity, multi-device synchronization, account ownership/RLS, shared save recovery or remote backup. Its match simulation and generated clubs/players are fictional and deterministic, not real-world football data. Substitutions, tactical player instructions, rival-club squad development, generated injuries/suspensions, contract negotiation/renewal, detailed NPC motives and the full living-world loop remain outside this slice. The complete legacy Dynasty acceptance contract is tracked in docs/DYNASTY_STATUS.md. Online rooms, multiplayer/realtime, server-authoritative Alias scoring, Telegram identity, Supabase card reads, and persistent Alias match history remain unimplemented. No production feature parity is claimed.
