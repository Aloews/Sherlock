# Football Dynasty — implementation status

## Current playable slice

This repository contains an independent Rust implementation. It does not import or connect to the legacy production database.

- Create a career with a manager, fictional club and optional heir.
- Start with 22 generated fictional players and a fixed three-club rival league.
- Select 4–3–3, 4–4–2 or 3–5–2 and adjust the starting eleven.
- Train the squad in balanced, attacking, defensive or fitness sessions; player attributes, fitness, morale and potential affect development.
- Buy from a generated transfer market, sell non-starters, and record cash movements in a club ledger.
- Simulate the six matchdays of a four-team season. The same campaign seed and match inputs produce the same match scores.
- Calculate standings from completed fixtures and update player appearances/goals.
- Advance seasons, award the champion, and transfer control to an adult heir while preserving club state.
- Automatically serialize and restore a campaign in browser local storage.

## Deliberate boundary

The mode is currently a local browser career. It is not cloud persistence, a Telegram Mini App session, or a multi-device account feature. A user clearing browser storage can lose the save. No legacy production Supabase project, secrets, tables or Vercel project are used.

All Dynasty teams and players in this slice are generated fiction. They must never be mixed with real player cards, player statistics, club rosters, forecasts or other real-world football datasets.

## Not yet accepted as complete

The original product map defines seven larger systems. The current slice does not yet meet the full acceptance criteria for:

1. Persisted squad/bench workflow with owner-scoped API and browser reload tests against a server.
2. Full player lifecycle: dated injuries, suspensions, recovery, contract expiry/renewal and development ceilings integrated with the calendar.
3. Tactical match engine with substitutions, detailed scorer/assist event attribution, player match statistics and realistic team selection.
4. Negotiated transfers, atomic dual-club roster changes, contract histories and complete transaction reconciliation.
5. Detailed cash-flow forecasting, wage/budget policies, board decisions and facility effects beyond the training-ground upgrade.
6. Rival-club squad management, AI decisions, a living league and richer season rollover.
7. The full knowledge-gated living-world event/action/reaction chain, persistent NPC schedules and delayed consequences.

Other missing release work includes authenticated server-side persistence, ownership isolation, cloud backup/restore, API/browser end-to-end journeys, load/security testing, and a production deployment containing this feature.

## Verification gate

Do not call the mode production-ready until the exact candidate SHA passes:

- \`cargo fmt --all -- --check\`
- \`cargo clippy --workspace --all-targets -- -D warnings\`
- \`cargo test --workspace --all-targets\`
- \`cargo check -p sherlock-web --target wasm32-unknown-unknown\`
- the Trunk browser bundle build
- browser journeys for create → reload → squad selection → training → transfer → matchday → season rollover → succession
- browser storage corruption/recovery and real-device responsive checks

Each gate proves only its own layer. A successful build is not proof of a working deployed career.

# Domain boundary: Football within the wider Sherlock world

Football Dynasty is a specialized domain module, not the definition of the entire Sherlock simulation. The long-term product direction is specified in [HISTORICAL_WORLD_SIMULATION.md](HISTORICAL_WORLD_SIMULATION.md).

In a future integrated world, football may interact with shared systems for population and labor, education and youth development, health and medicine, facilities and construction, energy and transport, finance and credit, media and broadcasting, law and regulation, politics and culture. These integrations are future work; they are not present in the current local career slice.

Keep three kinds of data separate:
1. Real-world football reference facts (with source and temporal provenance).
2. Fictional Football Dynasty campaign entities and outcomes.
3. Historical or generated whole-world simulation state.

A real-world club or player record must not be silently rewritten by a generated campaign. An alternate-history scenario may intentionally branch from reference facts, but the branch must be labelled as simulation. Football-specific rules should consume shared world concepts where appropriate rather than create competing definitions of money, population, time, geography or institutions.
