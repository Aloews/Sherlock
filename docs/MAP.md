# Sherlock — Project Map

## Public client
- Repository: `Aloews/Sherlock`.
- Frontend: Rust + Leptos compiled to WebAssembly.
- Football Alias: local same-device mode, core in `crates/game-core`.
- Football Dynasty: separate mode tab and Leptos client in `crates/web/src/dynasty.rs`.
- The previous Sherlock Scholes app, its production Vercel project, Supabase database and secrets are outside this repository's deployment boundary.

## Football Dynasty
- Backend: `services/football-dynasty`.
- Stack: Rust, Axum, Tokio, SQLx, PostgreSQL 17.
- Fictional data: isolated private `fd` schema; not shared with real-football or Alias domains.
- Implemented backend domains: campaign/character/family lifecycle, inheritance, club ownership, balanced journal, calendar, season and fixtures, match engine, player match statistics, living-world events and delayed reactions.
- The new Leptos UI is wired to the API contract for campaign creation/list/detail, heirs, calendar advance, event choices and season/standings/fixtures.
- Design and acceptance criteria: `docs/football-dynasty/`.

## Verification and release status
- Root Rust CI: format, Clippy, unit tests, WASM check, Trunk browser bundle, asset verification and release workspace build passed on the current feature branch commit.
- Football Dynasty CI: format, Clippy and PostgreSQL-backed campaign integration tests passed; final service release build is the remaining CI step.
- These checks run in GitHub Actions against disposable infrastructure; no production database has been migrated.
- No Dynasty backend service has been deployed and no dedicated runtime database or Telegram verifier has been configured.
- The UI is in the open draft PR and is not yet in the public production deployment.
- Do not call the mode playable/production-ready until API hosting, authenticated browser/Telegram smoke testing, save/reload and succession have all been verified.

## Next gates
1. Complete the service release build and keep both CI workflows green on the same commit.
2. Provision a separate PostgreSQL database and a new Telegram initData verifier/secret; do not reuse legacy production resources.
3. Deploy the API independently, configure `SHERLOCK_DYNASTY_API_URL` and exact allowed frontend origin, then verify health/readiness.
4. Merge and deploy the public client once the API is reachable.
5. Run authenticated browser/Telegram scenarios: create → reload → advance calendar → resolve event → season/match report → succession.
