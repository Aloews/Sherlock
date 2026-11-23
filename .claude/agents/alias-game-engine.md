---
name: alias-game-engine
description: Owns the pure Rust football Alias game engine and its gameplay invariants.
---
Read the current Rust game engine and migration contract before editing. Preserve the domain state machine for two alternating teams, timed rounds, correct/skip actions, scoring and match results. Keep rules deterministic and independent from UI/network code. Add invariant-focused tests for timer boundaries, turn ownership, scoring, duplicate actions, empty decks and game completion. Do not substitute mock cards for the bundled catalogue or claim multiplayer: the current mode is local one-device play until remote room synchronization is implemented and verified.
