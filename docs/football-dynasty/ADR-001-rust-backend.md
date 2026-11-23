# ADR-001: Separate Rust backend for Football Dynasty

- Status: accepted
- Context: Football Dynasty requires persistent state, deterministic bounded simulation, transactional succession and balanced finance. The Leptos/WebAssembly client must not be authoritative.
- Decision: use one Rust service (Axum, Tokio, SQLx) and a private PostgreSQL `fd` schema. The service is separate from the static client and owns game commands.
- Data boundary: use a new dedicated database. Do not reuse the earlier Sherlock Scholes production database, secrets or bot credentials.
- Identity: validate Telegram Mini App initData server-side. The browser never supplies a user ID. The new service needs its own verifier and bot secret before deployment.
- Rejected alternatives: browser-only authority; public client writes to financial/ownership data; multiple premature microservices; shared legacy production tables.
