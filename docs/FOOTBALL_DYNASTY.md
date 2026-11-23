# Football Dynasty — Rust rebuild

## Delivered local playable slice

The Dynasty mode is a separate product area in the Rust/Leptos client. Use the top navigation or open `/?mode=dynasty`.

The domain engine lives in `crates/dynasty-core`. It is pure Rust and has no database, network, clock, or LLM dependency. The client persists the serialized campaign in browser local storage under `sherlock.football-dynasty.v1`.

### Included systems

- Campaign setup with founder, house name, club name, origin and sandbox/scenario selection.
- A fictional four-club competition and one complete double round-robin season (six matchdays, twelve fixtures).
- Reproducible match results derived from campaign seed and fixture identity.
- A standings table derived from played fixtures.
- Day-based campaign calendar, dated event deadlines and weekly board/sponsor/supporter/scouting decisions.
- Club reputation and dynasty prestige consequences.
- Separate founder and club cash accounts, opening capital, and balanced journal entries. Accepting the sponsor decision credits €250,000 to the club through a balanced journal.
- Family member creation, heir eligibility, founder death at the implemented age boundary, succession, and a chronicle.
- Serialization round-trip tests and browser-local save/reload.

### Trust boundary and limitations

This build is a single-browser prototype. Local storage is user-editable and is not an authoritative record; it must not be used for real accounts, competitive ranking, multiplayer, or valuable assets. There is no login, remote API, database, cross-device synchronization, cloud backup, or server-side ownership validation in this slice.

The old repository's Rust API is intentionally not copied wholesale because its identity verifier and persistence layer are coupled to the old Sherlock/Supabase deployment. This rebuild must receive its own identity provider, database, migrations, owner isolation and secrets before cloud campaigns are enabled. No old production database or deployment is used by this mode.

Not yet ported: detailed player and staff career systems, tactical match control, contracts and transfer windows, multi-club holding operations, detailed ownership/equity, financial risks and tax, rival dynasty AI, political institutions, personal life simulation, spatial incidents and the full living-world consequence graph.

## Release gates

1. Cargo fmt, Clippy with warnings denied, unit and integration tests.
2. WASM build and Vercel artifact generation.
3. Manual browser verification of mode navigation, campaign creation, browser reload, matchday advancement, event resolution, family creation, age boundary and succession.
4. Before cloud persistence: independent auth verification, database isolation, server-authoritative commands, migrations tested against a disposable DB, audit/replay guarantees, backup and restore checks.
