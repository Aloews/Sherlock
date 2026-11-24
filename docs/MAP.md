# Project MAP — Sherlock Historical World Simulation

## North-star outcome

Deliver a persistent, inspectable simulation of human civilization in which ecology, geography, population, knowledge, production, trade, finance, organizations, power, dynasties, culture and historical memory interact over generations. Football is a specialized optional sector, not the product's organizing principle.

Authoritative concept: [PRODUCT_CONCEPT.md](PRODUCT_CONCEPT.md).

## Current repository reality (2026-11-24)

The repository is a Rust-first rebuild, but its current product is still a narrow local prototype:
- Axum API currently exposes a health endpoint.
- Leptos/WASM client contains local Alias and Football Dynasty experiences.
- Dynasty saves in browser local storage; it is not a server-authoritative world simulation.
- Current Dynasty teams/players and league simulation are fictional and deterministic.
- The existing workspace has no accepted civilization-scale world engine, historical evidence pipeline, persistent event log or integrated finance/institution/dynasty model.
- README and MIGRATION_SCOPE describe the earlier football-product migration. They are not the new product north star and must be reconciled without deleting useful compatibility requirements.

No production readiness or feature parity is implied by this map.

## Product boundaries

- The world engine is the source of truth for simulated state.
- Domain modules own rules, not UI components.
- Historical evidence is stored separately from generated simulation state.
- Football/real-world football data is a bounded specialized domain; it must not contaminate the universal economy, identity, time, resource or event abstractions.
- Legacy Sherlock Scholes compatibility work is a migration/adapter concern, not the core of the new simulation.
- Never alter the legacy production app/database as part of this rebuild without explicit release authorization.

## Execution order

### W0 — Foundations
- [ ] Establish domain glossary and units (time, money, quantity, energy, population).
- [ ] Define stable IDs, world identity, calendar, time scale and deterministic RNG.
- [ ] Define versioned event envelope, command model, event ordering and replay semantics.
- [ ] Define provenance/evidence types: source fact, derived estimate, assumption, generated outcome, disputed claim.
- [ ] Define snapshot, event log, entity history and persistence contracts.
- [ ] Add invariant/property-test harness and small deterministic reference world.

### W1 — First playable systemic vertical slice
- [ ] Model geography/resource locations, resource stocks/flows, settlements and population cohorts.
- [ ] Model households, labor, production sites, goods, inventories, transport and trade.
- [ ] Model treasury, ledger and prices with explicit accounting invariants.
- [ ] Advance multiple ticks and generations; inspect causes and replay from seed.
- [ ] Persist/reload a world through PostgreSQL; no browser-only save as authoritative state.

### W2 — Economic institutions
- [ ] Firms, banks, ownership, capital, credit, collateral, debt, interest and defaults.
- [ ] Investment, productivity, wages, taxes, public goods and distribution.
- [ ] Financial fragility, bubbles, contagion and crisis as emergent behavior.
- [ ] Auditable balance sheets and transaction ledger.

### W3 — Power and institutions
- [ ] States, laws, courts, enforcement, legitimacy, political coalitions and diplomacy.
- [ ] Institutional capture, corruption, conflict, regulation and public finance.
- [ ] Feedback between wealth, power, trust and institutions.

### W4 — Generational society and memory
- [ ] People, households, families, dynasties, inheritance and succession.
- [ ] Education, skills, reputation, social mobility and networks.
- [ ] Culture, identity, myths, narratives, media and expectations.
- [ ] Archives, historical memory, institutional learning and knowledge loss.

### W5 — Technology, environment and sectors
- [ ] Research, invention, diffusion, infrastructure and energy transitions.
- [ ] Climate/ecological feedback, resource depletion and restoration.
- [ ] Plug-in specialized sectors; football is one example, not a privileged core.

### W6 — Product experience
- [ ] Scenario creation and world selection.
- [ ] Observe/investigate/intervene loop and meaningful player agency.
- [ ] Causal-chain explorer, entity histories, ledgers and evidence browser.
- [ ] Save/restore, resilience, accessibility, responsive web/mobile UX.

## Engineering gates

For every vertical slice:
1. Define acceptance criteria before implementation.
2. Inspect existing implementation and preserve only reusable, compatible components.
3. Add domain logic behind typed interfaces; no business rules in UI.
4. Test invariants, edge cases, deterministic replay and persistence.
5. Run format, lint, unit, integration and browser tests on the exact candidate SHA.
6. Inspect CI results and runtime evidence; fix failures before merge.
7. Record evidence, limitations and next executable tasks here.

## First milestone definition

A small world can be created with a fixed seed; it contains resource-bearing locations, at least two settlements, population, production, inventories, trade and a treasury. The world advances through explicit ticks. Each mutation is explained by a versioned event with actor, cause, affected entities, time and provenance. Replaying the same seed and commands reproduces the same result. Snapshot persistence and reload preserve state and history. Tests verify resource non-negativity, ledger balance, inventory accounting and deterministic replay.

Do not expand into dynasties, politics, culture or football until this vertical slice is correct and observable.
