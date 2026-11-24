# Historical World Simulation — product and domain specification

**Status:** proposed domain architecture; documentation only, not implemented.
**Scope:** the whole Sherlock world simulation. Football is one specialized domain inside the world, not the definition or boundary of the world.
**Implementation target:** Rust workspace; integrate incrementally with existing `game-core` and `dynasty-core` without breaking current modes.

## 1. Product definition

Sherlock is intended to become a persistent, systemic world simulation in which geography, ecology, population, culture, technology, production, trade, finance, institutions, politics, conflict and individual lives continuously influence one another across generations.

The world is not a football manager with decorative countries around it. Football, clubs, players, competitions, transfers, stadiums, supporters and football data are one domain-specific layer of a much larger world. Depending on the scenario and era, other domains may be more important: agriculture, mining, metallurgy, energy, shipping, construction, manufacturing, science, religion, education, medicine, media, finance, military organization and public administration.

The simulation must support:
- historically grounded scenarios and explicitly labelled alternate histories;
- fictional/generated worlds using the same systemic rules;
- different starting dates, regions, scales and playable roles;
- autonomous agents and institutions with bounded knowledge, goals, incentives and constraints;
- emergent events rather than a fixed sequence of scripted crises;
- persistent records, provenance and causal explanations for important outcomes.

The goal is not to claim that history is deterministic. The model should make causal assumptions explicit, expose uncertainty, and allow players to test plausible alternatives.

## 2. Core design principles

1. **World first, domains second.** A shared world state owns time, geography, population, resources, institutions and event history. Domain modules add specialized rules; none owns the whole simulation.
2. **Football is a first-class domain, not the root domain.** Football entities connect to population, labor, education, health, transport, media, construction, energy, finance, law, politics and culture. Real football datasets enrich this domain but do not define the simulated world's complete population or economy.
3. **Two data realities stay distinct.** Historical/real-world reference data and generated simulation state must never be silently mixed. Every imported fact has source, timestamp, scope, units and confidence. Generated outcomes are labelled as simulation.
4. **History is evidence-constrained, not a deterministic script.** Historical scenarios may start from documented facts. Later simulated events can diverge. The system must mark the divergence and not present counterfactual results as historical fact.
5. **No single-variable explanations.** Geography creates constraints and opportunities, not destiny. Technology requires complementary inputs and institutions. Age is not a proxy for ideology or competence. Dynasties do not automatically preserve knowledge.
6. **Causal links are inspectable.** Important outcomes should retain contributing events, state changes, rules and uncertainty. Correlation must not be labelled causation.
7. **Different clocks.** Matches and logistics may run hourly/daily; markets and politics daily/monthly; demographics and technological diffusion yearly/generationally. Systems advance at appropriate resolutions without pretending every process changes at the same speed.
8. **Deterministic replay where feasible.** Given the same seed, ruleset version and ordered inputs, a simulation run should be reproducible. Randomness must be seeded and recorded.
9. **Modular depth.** A scenario can enable high-detail simulation for selected domains and aggregate others. The world remains coherent when a domain is abstracted.
10. **No unbounded agent autonomy.** Agents act from partial information, limited resources and explicit decision rules. They do not know hidden world state or future events.

## 3. World model: shared layers

### 3.1 Space and environment
- Planet/world map, coordinates, terrain, elevation, waterways, coastlines, climate zones and seasonal patterns.
- Natural hazards and long-term environmental change.
- Resource deposits, renewability, extraction cost, accessibility and depletion.
- Settlements, population density, land use, ports, roads, rail, air routes and communication networks.
- Travel time, transport capacity, chokepoints, route security and maintenance.

Geography should affect yields, costs, disease exposure, connectivity, defense and settlement patterns through explicit rules. It must not hard-code that a river or port guarantees wealth.

### 3.2 Population and agents
- Individuals, households, communities, professions, social strata and migration groups.
- Birth, aging, education, health, work, income, wealth, beliefs, relationships and death.
- Skills and experience are separate from age.
- Agents have local knowledge, imperfect beliefs, memory, preferences, obligations and bounded decision capacity.
- Cohorts are defined by formative exposure and shared historical conditions, not only by labels such as “generation X”.

### 3.3 Culture, knowledge and narratives
- Languages, religions, customs, laws, education, scientific traditions, media and social norms.
- Beliefs and myths are modeled as socially transmitted narratives that may legitimize authority, coordinate action, shape trust, risk appetite or collective identity.
- Narratives can be true, false, contested, symbolic or changing. Their popularity is not proof of factual truth.
- Knowledge has forms: tacit skill, written records, oral tradition, institutional procedure, scientific theory and practical know-how.
- Transmission can fail through destruction, censorship, migration, language loss, institutional collapse, secrecy, skill bottlenecks or deliberate forgetting.

### 3.4 Technology and production
Model the chain as a network rather than a mandatory linear ladder:

`resources + energy + skills + tools + capital + institutions + demand -> production capability`

A technology record includes prerequisites, inputs, process, output, efficiency, reliability, scale, maintenance, externalities, skills, diffusion channels, adoption cost and source/confidence. Invention, prototype, commercial adoption and broad diffusion are separate states.

Production domains can include food, textiles, metals, ceramics, construction, shipbuilding, transport, energy, chemicals, machinery, electronics, software, media and services. Football production includes player development, coaching, facilities, medical services, equipment, broadcasting and event operations; it consumes labor, capital, land, energy and transport capacity like other sectors.

### 3.5 Markets, money and finance
- Barter and commodity exchange where appropriate; money forms, monetary authorities and payment infrastructure.
- Prices, wages, rents, interest, credit, debt, collateral, insurance, equity, public finance, taxation and trade balances.
- Banks, merchants, exchanges, funds, households, states, corporations and informal lenders.
- Leverage, liquidity, maturity mismatch, concentration, defaults, bailouts, inflation and distribution of losses.
- Asset-price boom/bust is not automatically a “bubble”: classification requires evidence of valuation, leverage, expectations and market behavior. Store competing interpretations where historians/economists disagree.

### 3.6 Institutions, states and power
- Families/dynasties, councils, monarchies, republics, empires, city-states, bureaucracies, courts, religious institutions, guilds, corporations, banks, universities and professional bodies.
- Legitimacy, succession, administrative capacity, property rights, enforcement, corruption, taxation, public goods and coercive capacity.
- Institutions have memory, procedures, staff, archives, incentives and failure modes. They can outlive rulers, but can also decay or be captured.

### 3.7 Specialized domains
Domain modules plug into shared world services through stable contracts. Initial examples:
- football and other sports;
- agriculture and food systems;
- mining, metallurgy and energy;
- manufacturing and construction;
- shipping, logistics and trade;
- banking, capital markets and public finance;
- science, education and medicine;
- religion, culture and media;
- military and diplomacy.

A domain may be detailed or aggregated depending on scenario scale. Domain modules must not duplicate canonical concepts such as population, money, geography or time.

## 4. Dynasties and financial memory

A dynasty is one possible carrier of knowledge, not a magical repository and not necessarily the best one.

Represent knowledge as records and capabilities held by people and institutions:
- who knows it and how they learned it;
- whether it is tacit, written, ritualized or encoded in procedure;
- where it is stored and who can access it;
- whether it is taught, copied, translated or practiced;
- its reliability, age, provenance and uncertainty;
- whether a succession crisis, purge, war, migration or institutional reform can destroy access.

Memory carriers include ruling houses, merchant families, treasuries, archives, bureaucracies, temples, religious networks, guilds, banks, universities, military institutions, courts and central banks.

Dynastic continuity can preserve personal networks, strategic priorities, private correspondence and intergenerational experience. It can also cause nepotism, information filtering, succession conflict, hoarding, and loss when a line ends. Bureaucratic and professional institutions can preserve routines beyond a ruler's life, but may become rigid, fragmented or politically captured.

The simulation should compare these mechanisms rather than assume one is superior. A “financial memory” capability can reduce repeated errors only when knowledge is accessible, trusted, relevant to current conditions and acted upon. Remembering a past crisis does not guarantee avoiding the next one.

## 5. Generations and age-period-cohort effects

Keep three variables distinct:
- **Age/life-cycle:** changing resources, responsibilities, experience and risk capacity across a person's life.
- **Period:** shocks that affect many people at the same time, such as war, inflation, pandemic or credit contraction.
- **Cohort/formative experience:** durable effects of conditions experienced during formative years or career entry.

Do not assign deterministic traits to age bands or generations. Risk-taking depends on wealth, role, incentives, experience, social networks, information and institutional constraints. A 25-year-old retail investor and a 25-year-old central banker do not occupy equivalent positions.

Cohort effects may alter beliefs and behavior, but the model should require scenario-specific evidence or clearly labelled assumptions before applying them.

## 6. Myths, ideology and collective narratives

Narratives can coordinate expectations and justify institutions. They may influence:
- perceived legitimacy of rulers and institutions;
- trust in money, contracts, markets and public authority;
- willingness to migrate, invest, borrow, fight, innovate or adopt a technology;
- social status and consumption;
- tolerance for inequality, taxation or sacrifice.

Narrative strength is not the same as truth. Model audience segments, transmission channels, credibility, competing narratives, counter-evidence and changes over time. A ruler may sponsor a myth, but reception is not guaranteed. Religious and cultural beliefs must not be reduced to a single economic utility score.

## 7. Historical evidence and scenario modes

Use explicit epistemic labels:
- **Documented:** directly supported by primary sources or strong scholarly consensus.
- **Corroborated:** supported by multiple independent sources, with some uncertainty.
- **Estimated:** quantitative reconstruction from incomplete evidence.
- **Interpretation:** a scholarly explanation among competing accounts.
- **Hypothesis:** plausible but not established.
- **Counterfactual/simulated:** generated by game rules, not a claim about actual history.
- **Unknown:** evidence insufficient.

Every historical dataset should carry citation/provenance metadata, temporal validity, geographic scope, units, translation/normalization notes and confidence. Contradictory sources should be retained as competing claims rather than silently flattened.

Scenario modes:
1. **Historical reconstruction:** start from evidence-based initial conditions; report uncertainty.
2. **Alternate history:** start from historical conditions but allow divergence; label it clearly.
3. **Generated world:** coherent procedural geography, institutions, cultures and histories; do not imply it is real history.
4. **Hybrid:** real geography and selected historical data combined with fictional actors or events, with provenance visible.

## 8. Emergent systemic loop

The following is a useful causal hypothesis, not a mandatory script:

environment/geography -> resource access and connectivity -> production and settlement -> technology and skills -> trade and information -> institutions and capital -> narratives and expectations -> investment and collective behavior -> growth, concentration or fragility -> shock/crisis -> political response and institutional change -> memory transmission or loss -> later decisions.

Feedback runs in both directions. For example, finance funds infrastructure; infrastructure changes geography's effective accessibility; narratives affect policy; policy changes technology adoption; war redirects production; migration transfers skills; institutions shape which inventions spread.

Crises should emerge from interacting conditions (e.g. leverage, liquidity mismatch, correlated exposures, supply shocks, political conflict, harvest failure), not from a hidden “bubble timer”.

## 9. Time, scale and simulation orchestration

- Use a world calendar with scenario-specific calendar systems and explicit conversion to a canonical timeline.
- Separate event time, observation time and record time.
- Support local time steps and aggregated updates.
- Define update ordering and conflict resolution so modules cannot produce order-dependent accidental outcomes.
- Use event queues for discrete events and state transitions for continuous/periodic processes.
- Allow pausing, fast-forwarding, replay and inspection.
- Record ruleset/schema versions and random seeds for reproducibility.
- Make expensive domains opt-in or lower fidelity at broad scales; do not simulate every person at full detail in every scenario.

## 10. Architecture direction for the current Rust repository

Current observed workspace has `game-core`, `dynasty-core`, `api` and `web`. Preserve working Alias and Football Dynasty behavior.

Proposed incremental boundaries (names are conceptual until accepted by code):
- `world-core`: world identity, calendar, map references, event log, simulation clock, seed and shared IDs.
- `geography-core`: terrain, climate, resources, routes and settlement suitability.
- `population-core`: individuals, households, cohorts, migration and demographics.
- `culture-core`: languages, beliefs, narratives, education and knowledge transmission.
- `technology-core`: inventions, prerequisites, diffusion and adoption.
- `production-core`: sectors, facilities, inputs, outputs, energy and labor.
- `economy-core`: goods, prices, money, trade, credit, public finance and markets.
- `institutions-core`: organizations, state capacity, dynasties, archives and succession.
- `history-core`: source-backed claims, chronologies, scenario definitions and uncertainty.
- `football-domain`: clubs, players, competitions, contracts, matches, supporters and football-specific rules; consumes shared world services.
- `game-core`: game/session orchestration and player-facing actions, not the owner of all world-domain rules.
- `api` and `web`: delivery and presentation only; no duplicate business rules.

Do not create all crates at once. First define stable shared IDs, calendar, event envelope, provenance, serialization/versioning and deterministic update contracts. Then extract one vertical slice and test it end to end.

### Integration with existing Dynasty

The current `dynasty-core` is a football career slice with fictional clubs/players, club finances and family succession. It must remain playable during migration. Treat it as a bounded context that can later consume world-level population, labor, finance, facilities, media, transport and law. Its existing player ages, contracts, ledger, chronicle and heirs are useful starting concepts but are not yet a general population, economy or dynasty-memory implementation.

Real football data remains a separate reference-data integration with provenance and temporal validity. Never substitute fictional generated data for real football facts or vice versa. A club's in-world economic effects may interact with the wider simulated economy, while the factual record of a real club remains read-only reference material unless the scenario explicitly enters alternate-history mode.

## 11. Initial implementation roadmap

### Phase 0 — contracts and safeguards
- Define world ID, entity ID, calendar/time, event envelope, provenance, confidence labels, seed/replay contract and schema versioning.
- Document historical vs simulated vs real-football reference data boundaries.
- Add architecture decision records and acceptance tests before domain extraction.

### Phase 1 — smallest coherent world slice
Build one small map with terrain, climate bands, one renewable and one non-renewable resource, settlements, population, one production chain, one trade route, money/ledger and one institution. Simulate multiple years and show causal event history. No giant map or detailed individual simulation yet.

### Phase 2 — continuity and knowledge
Add households, careers, succession, institutions, archives, skills and transmission/loss of knowledge. Compare a ruling family with a durable bureaucracy and merchant institution under the same crisis.

### Phase 3 — technology and narratives
Add technology prerequisites/diffusion and competing narratives with measurable but bounded effects on beliefs and collective action.

### Phase 4 — macro-financial dynamics
Add credit, leverage, collateral, liquidity, public debt, inflation and crisis transmission. Validate against stylized historical cases without claiming the model reproduces history exactly.

### Phase 5 — domain integration
Connect football as one economic/cultural sector: player labor markets, youth development, clubs, facilities, stadium construction, broadcasting, sponsorship, supporter communities, transport, media, public investment and regulation. Preserve a strict separation between historical football facts and generated campaign outcomes.

### Phase 6 — scale, scenarios and content
Add scenario tooling, source-backed historical starts, generated worlds, alternate-history branching, AI agent policies, performance profiling and richer domain modules.

## 12. Acceptance criteria

The architecture is not accepted until:
- a world can be created with stable IDs, seed, calendar and versioned state;
- at least two domain modules exchange typed events/state without circular ownership;
- a multi-year run is deterministic under the same seed, inputs and ruleset;
- provenance and uncertainty survive serialization and UI/API presentation;
- a player can inspect why a major event occurred and distinguish evidence from simulation;
- geography affects costs/opportunities through tested rules, not hard-coded outcomes;
- technology adoption checks prerequisites, inputs, skills, cost and diffusion;
- knowledge can be held by different carriers and can be transferred, lost or become inaccessible;
- age, period and cohort effects are not conflated;
- historical reference data, generated world data and real football data remain distinct;
- existing Alias and Football Dynasty tests and user journeys remain green;
- performance tests establish explicit limits for population and time-step scale.

## 13. Non-goals and anti-patterns

- No claim that all financial crises are planned by dynasties or elites.
- No universal “generation creates bubble” law.
- No assumption that all technologies produce progress or that all innovation requires a bubble.
- No geographic determinism.
- No treating myth, religion or ideology as falsehood by default.
- No universal single currency, nation-state or modern corporation projected into eras where they did not exist.
- No fake precision for ancient populations, ages, prices or output.
- No automatic conversion of every historical downturn into a financial bubble.
- No all-at-once rewrite of the current Rust workspace.
- No production-ready claim based only on compilation or unit tests.

## 14. Open design decisions

These require explicit product/engineering decisions before deep implementation:
1. Primary player role(s): dynasty leader, state leader, merchant, household, institution, or observer/sandbox.
2. Default scenario scale and time resolution.
3. Real historical timeline coverage vs generated world as the default.
4. Whether the player can switch roles or own multiple institutions.
5. World-map representation and map data licensing.
6. Population simulation fidelity and performance budget.
7. AI-agent architecture and policy transparency.
8. Multiplayer and shared-world consistency requirements.
9. How historical source citations appear in the UI.
10. Which vertical slice best proves the architecture without overbuilding.
