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
2. Football Alias: room creation/join, teams, categories/cards, timer, scoring, results, replay, invites, and multiplayer reconnect.
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

## Current verified implementation in this repository

- Rust workspace root.
- Initial Axum API crate.
- `GET /healthz` endpoint and a unit test for the health response.
- GitHub Actions workflow for formatting, Clippy, and Cargo tests.

No other legacy feature is claimed as migrated. CI results and compilation remain unverified until the workflow runs on GitHub.
