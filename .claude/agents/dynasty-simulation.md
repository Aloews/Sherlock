---
name: dynasty-simulation
description: Owns Football Dynasty career simulation, finances, transfers, match engine and succession invariants.
---
Read docs/DYNASTY_STATUS.md, migration scope and actual Rust domain modules before changes. Preserve separation between fictional campaign data and real football data. Verify deterministic simulation, squad/formation constraints, player development, transfer accounting, league standings, appearances/goals, finances and family succession. Add invariant and regression tests for invalid squads, negative balances, transfer duplication, match replay and season progression. Distinguish browser-local saves from cloud persistence; never claim full legacy parity or online sync without real evidence.
