# Sherlock — Historical World Simulation

## Product definition

Sherlock is a persistent, systemic simulation of human civilization across geography, ecology, population, technology, production, trade, finance, institutions, political power, culture, collective beliefs and historical memory. It is not primarily a football manager, stock-trading game, dynasty clicker, or scripted historical campaign.

The world is the primary product. Football and real-world football data are optional specialized domains that can participate in the same world through organizations, labor, capital, audiences, infrastructure, media and culture. They must not define the simulation's core architecture.

## Core design principles

1. **One world, connected systems.** Every domain acts on shared entities and resources; avoid isolated minigames and duplicated economies.
2. **Emergence over scripts.** Outcomes arise from rules, constraints, decisions, information and shocks. Historical events may seed initial conditions, but the future is not a fixed script.
3. **Causal continuity.** Every material change has a cause, time, actor, affected entity and auditable record.
4. **Generational time.** People, families, firms, banks, states and institutions have lifecycles, succession, accumulated capital, obligations and memory.
5. **Epistemic limits.** Agents act on what they know or believe, not omniscient world state. Rumors, expectations, myths and incomplete information can affect decisions.
6. **Historical grounding.** Historical data must carry provenance, dates, uncertainty and distinctions between documented fact, scholarly interpretation, model assumption and generated simulation.
7. **No fake reality.** Simulated or generated entities/events must never be presented as verified historical facts or current real-world data.
8. **Inspectable systems.** Players and developers can trace why a price, crisis, migration, institution, dynasty or political outcome changed.
9. **Deterministic replay where practical.** Simulation inputs, ruleset versions and random seeds are recorded so a world can be reproduced and debugged.
10. **Modular specialization.** Domain modules (including football) integrate through shared contracts rather than owning parallel world economies.

## World model

The simulation is organized around connected layers:

- **Planet and ecology:** geography, climate, water, soil, minerals, energy, biodiversity, hazards and environmental change.
- **People and settlements:** individuals, households, skills, health, education, migration, birth/death, cities and social groups.
- **Knowledge and technology:** discovery, research, diffusion, education, infrastructure, production methods and technological constraints.
- **Production and exchange:** land, labor, capital, firms, supply chains, goods, services, logistics, trade routes and prices.
- **Money and finance:** units of account, credit, debt, collateral, interest, banks, payment systems, investment, liquidity, defaults, bubbles and crises.
- **Organizations and institutions:** firms, guilds, banks, universities, religious/cultural bodies, courts, states, regulators and rules.
- **Power and politics:** authority, legitimacy, coalitions, taxation, public goods, conflict, diplomacy, capture and institutional change.
- **Dynasties and intergenerational capital:** family networks, inheritance, marriage alliances, ownership, reputation, education, influence, succession and decline.
- **Culture and shared belief:** language, identity, religion, myths, narratives, status, trust, norms, media and collective expectations.
- **Historical memory:** archives, records, education, institutional learning, forgotten knowledge, trauma, narratives and reinterpretation.
- **Specialized sectors:** football and other industries, each using shared population, labor, capital, infrastructure, finance, organizations and cultural systems.

## Core causal loop

A baseline systemic loop to be implemented and validated:

resources and environment → settlement and population → skills and knowledge → production and exchange → surplus and wealth distribution → credit and investment → organizations and institutions → control and political power → expectations, narratives and trust → new investment or extraction → growth, fragility, crisis or adaptation → institutional and generational memory.

This is a design hypothesis, not a claim that every historical society follows one universal deterministic sequence. Rules must permit divergent outcomes and feedback in both directions.

## Player role and experience

The player observes, investigates and intervenes in a living world. The exact campaign role (individual, household, organization, dynasty, state or analyst) is a scenario/configuration, not a hard-coded assumption in the world engine. Actions consume resources, require access/information, create obligations and can have delayed or unintended consequences. The simulation should make causal chains legible without turning outcomes into predetermined quest scripts.

## Simulation time

Support configurable time scales and pause/step/fast-forward. Long-horizon processes (demography, climate, institutional change, technology) and short-horizon processes (markets, logistics, political decisions, matches) must not be forced into one tick rate. Use explicit scheduling and event ordering, with versioned rules and replayable seeds.

## Data and evidence model

Historical imports require source/provenance metadata, valid-time intervals, observed/estimated status, confidence or uncertainty where available, and a record of transformations. Keep source facts, derived estimates, model assumptions and generated simulation outcomes distinct in storage and UI. Conflicting sources must remain representable rather than silently collapsed.

## Architecture direction

- Rust-first domain engine with pure, testable domain logic.
- Separate bounded contexts connected by typed commands/events and shared identity/time/resource contracts.
- PostgreSQL persistence for world snapshots, event log, entity histories, ownership and provenance.
- API and client are adapters; simulation rules must not live in UI handlers.
- Deterministic seeded simulation and replayable event processing.
- Versioned schemas, rulesets and migrations; no destructive rewrite of legacy production data.
- No production secrets in this public repository.

## Delivery sequence

### Phase 0 — contracts and vertical slice
Define canonical time, IDs, units, ownership, event envelope, provenance, deterministic RNG and world snapshot contracts. Build a tiny test world with geography/resources, settlements/population, production/trade and a treasury. Advance it through several ticks, persist/reload it, and explain every state change from its event log.

### Phase 1 — material world
Add geography, resource stocks/flows, environment, population cohorts, households, settlements, labor, production, inventories, logistics and exchange. Validate conservation and accounting invariants.

### Phase 2 — finance and organizations
Add money, prices, credit, collateral, debt service, banks, firms, ownership, taxation, insolvency, investment and balance-sheet consistency. Model bubbles and crises as emergent feedback, not scripted labels.

### Phase 3 — power and institutions
Add states, laws, enforcement, public goods, political coalitions, institutional capture, legitimacy, conflict and diplomacy.

### Phase 4 — generations, culture and memory
Add individuals/families, succession, inheritance, networks, education, reputation, collective beliefs, archives and institutional learning.

### Phase 5 — technology and specialized sectors
Add research/diffusion, media and specialized industries. Integrate football only as one sector using shared world primitives; real football data remains a separate sourced dataset and must not be confused with generated historical-world entities.

### Phase 6 — player experience and content
Build scenarios, investigation tools, intervention UI, causal-chain visualization, historical source browser, save/restore, multiplayer only if separately justified, and accessibility/localization.

## Acceptance gates

A phase is accepted only when:
- domain invariants and property tests pass;
- deterministic replay reproduces the same state for the same ruleset, seed and commands;
- persistence round-trips preserve state and event history;
- causality/provenance can be inspected;
- no simulated output is mislabeled as historical fact;
- integration tests cover real persistence/API boundaries;
- UI journeys exercise actual domain behavior, not fixtures standing in for production;
- performance is measured against declared world-size and time-horizon targets.

A compile, mock demo, static screenshot or health endpoint is not product acceptance.
