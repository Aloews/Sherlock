# Football Dynasty API

Standalone Rust/Axum backend for the Football Dynasty mode in the new Sherlock repository.

## Local development

1. Start disposable PostgreSQL 17 with `docker compose up -d postgres`.
2. Set `DATABASE_URL=postgres://postgres:postgres@localhost:54329/football_dynasty`.
3. Run `cargo test --all-targets` from this directory. The integration test creates its own test-only Telegram verifier stub and applies the bundled SQLx migrations.
4. Run the API with the environment settings below and a separately provisioned secure Telegram identity-verification function.

Never point local testing at a production database.

## Security boundary

Requests to campaign routes require:
- `x-tg-init-data`: Telegram Mini App initData, verified server-side through `public.tg_validate_init_data(text)`.
- `Idempotency-Key`: a unique 8–128 character key for every mutation.

The browser never supplies a Telegram user ID. The verifier must use a bot token stored only in the new backend's secret store. The integration test installs a deliberately rejecting stub; it is test-only and must not be deployed.

## Database isolation

This service is designed for a dedicated PostgreSQL database. It creates a private `fd` schema and least-privilege `fd_runtime` role. Do not connect it to the older Sherlock Scholes production database, and do not reuse its service-role key, database credentials or bot token.

Before deployment, provide a restricted DB login and implement/configure `public.tg_validate_init_data(text)` securely for the new Telegram app. The schema migration only grants EXECUTE on that function if it exists; it does not manufacture a verifier or a bot secret.

## Runtime settings

- `FD_BIND_ADDR`: optional bind address; defaults to `0.0.0.0:$PORT` (8080).
- `FD_DATABASE_URL`: restricted runtime database URL. Prefer this over a privileged migration login.
- `POSTGRES_URL`: accepted pooler fallback.
- `POSTGRES_URL_NON_POOLING`: accepted direct/local fallback.
- `FD_MIGRATION_DATABASE_URL`: privileged migration URL, required only when `FD_RUN_MIGRATIONS=true`.
- `FD_ALLOWED_ORIGIN`: exact frontend origin allowed by CORS.
- `FD_MAX_CONNECTIONS`: bounded SQLx pool size.
- `FD_RUN_MIGRATIONS`: defaults to false; keep false on deployed replicas.

Apply reviewed migrations through a controlled path before switching traffic. Never put database or bot credentials in the frontend.

## API v1

- `GET /health/live`
- `GET /health/ready`
- `GET /api/v1/campaigns`
- `POST /api/v1/campaigns`
- `GET /api/v1/campaigns/{campaign_id}`
- `POST /api/v1/campaigns/{campaign_id}/heirs`
- `POST /api/v1/campaigns/{campaign_id}/time/advance` (1–31 in-game days)
- `GET /api/v1/campaigns/{campaign_id}/events`
- `POST /api/v1/campaigns/{campaign_id}/events/{event_id}/resolve`
- `GET /api/v1/campaigns/{campaign_id}/season`
- `GET /api/v1/campaigns/{campaign_id}/fixtures/{fixture_id}`
- `GET /api/v1/campaigns/{campaign_id}/world/events`
- `POST /api/v1/campaigns/{campaign_id}/world/events/{event_id}/respond`
- `GET /api/v1/campaigns/{campaign_id}/succession/preview`
- `POST /api/v1/campaigns/{campaign_id}/succession`

The old year-at-a-time route deliberately returns a conflict; campaign time advances in bounded day-level steps.

## Current scope

A single-player fictional campaign can create a founder, dynasty, club and balanced opening ledger; add heirs; progress the campaign calendar; resolve weekly decisions; complete a deterministic first league season; inspect match reports and player match events; react to the initial persisted world incident; and continue as the next generation after succession.

This is a backend implementation slice, not a fully deployed or browser-integrated game. The Leptos UI, separate database, identity configuration, API hosting and authenticated production smoke test remain release gates.
