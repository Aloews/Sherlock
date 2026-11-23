# Sherlock — Project Map

## Public client
- Repository: `Aloews/Sherlock`.
- Frontend: Rust + Leptos compiled to WebAssembly.
- Current implemented feature: local same-device Football Alias.
- Alias core: `crates/game-core`.
- The previous Sherlock Scholes app, its production Vercel project, Supabase database and secrets are outside this repository's deployment boundary.

## Football Dynasty
- Backend: `services/football-dynasty`.
- Stack: Rust, Axum, Tokio, SQLx, PostgreSQL 17.
- Fictional data: isolated private `fd` schema; not shared with real-football or Alias domains.
- Imported areas: campaign/character/family lifecycle, inheritance, club ownership, balanced journal, calendar, season and fixtures, match engine, player match statistics, living-world events and delayed reactions.
- Design and acceptance criteria: `docs/football-dynasty/`.

## Verification and release status
- Code import is not considered verified until the PostgreSQL CI workflow passes on the exact commit.
- No production database has been migrated.
- No backend service has been deployed.
- The public Leptos client is not yet connected to the Dynasty API.
- Do not call the mode playable/production-ready until an authenticated browser/Telegram smoke test proves create → save/reload → calendar → fixtures/events → succession.

## Next gates
1. Green Rust + PostgreSQL integration CI.
2. RLS and cross-owner isolation audit.
3. Leptos screen and API integration.
4. Separate database, credentials and identity verifier.
5. API deployment and end-to-end smoke test.
