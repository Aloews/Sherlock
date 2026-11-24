# Sherlock — repository agent contract

Read CLAUDE.md, README.md, docs/MIGRATION_SCOPE.md, docs/DYNASTY_STATUS.md and relevant source/tests before changes. This repository is a separate Rust-first rebuild; the legacy production application and its data are not modified here.

## Product invariants
- Preserve the Rust workspace and domain boundaries.
- Keep Alias game rules deterministic and separated from UI/network code.
- Keep Football Dynasty fictional career data separate from real football data.
- Local browser saves are not cloud persistence. Do not claim multiplayer, Telegram auth, remote Supabase reads or full legacy parity unless implemented and verified.
- Do not replace real product behavior with demos or mocks.
- User-facing UI must remain responsive, accessible and connected to real state.

## Verification
Run cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --workspace --all-targets, wasm target checks and browser/runtime smoke tests as relevant. Verify exact-head CI and inspect raw failures. A passing build is not product acceptance.

## Safety
Use focused branches and draft PRs. Do not merge, deploy, mutate production data, expose secrets or incur paid provider costs without explicit authorization. Report CODE_VERIFIED, CI, runtime and production evidence separately. If actual subagents cannot be launched, work sequentially and say so.
