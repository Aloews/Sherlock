# Sherlock Agent Operating Contract

This repository is building a historical world simulation, not extending a football manager. The north star and current execution order are defined in `docs/PRODUCT_CONCEPT.md` and `docs/MAP.md`.

## Team topology

- **Systems Architect** — owns bounded contexts, contracts, event semantics, time, units, determinism and dependency direction.
- **World Systems Researcher** — translates historical/economic scholarship into explicit, testable mechanisms; records evidence, uncertainty and model assumptions separately.
- **Rust Domain Engineer** — implements pure domain logic in typed Rust modules; no business rules in UI handlers.
- **Persistence Engineer** — owns PostgreSQL schema, event/snapshot persistence, migrations and transaction boundaries. No production DB changes without an approved migration plan.
- **Simulation QA / Red Team** — writes invariants, property tests, deterministic replay tests, adversarial scenarios and performance checks; does not approve its own implementation.
- **Product Integrator** — integrates reviewed work, updates MAP, checks full journeys and release gates.

## Mandatory rules

1. Read PRODUCT_CONCEPT.md, MAP.md and the relevant domain status before changing code.
2. Claim only work evidenced by a commit, passing check or observed runtime behavior.
3. No secrets, credentials, personal data or production data in this public repository.
4. Never modify or deploy the legacy production application/database as part of this rebuild.
5. Never mix sourced historical facts, estimates, assumptions and generated simulation outcomes.
6. Every state-changing command must validate preconditions and either commit atomically or leave state unchanged.
7. Record actor, tick/time, cause, affected entities, ruleset version and provenance for material changes.
8. Keep simulation deterministic for a fixed seed, ruleset version and command sequence wherever feasible.
9. Do not create parallel implementations of money, time, identity, ownership or resources in feature modules.
10. Every task must state acceptance criteria and tests. CI must pass on the exact candidate SHA.
11. Agents work on isolated branches or non-overlapping files. Integrate by PR; no force-push to main.
12. Reviewer and QA roles must challenge assumptions, not rubber-stamp the author.

## Handoff format

Each agent reports: files changed; design decisions; assumptions/evidence; tests run with exact commands and results; unresolved risks; next smallest task. A task is not complete because an agent says it is complete.

## Current execution boundary

The first accepted milestone is a small persistent world with resource-bearing locations, at least two settlements, population, production, inventories, trade and treasury. It must advance by explicit ticks, emit inspectable causal events, replay deterministically, and survive database snapshot/reload. Do not jump ahead to a giant feature list before this vertical slice is sound.
