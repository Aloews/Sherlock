# Sherlock — Product and Engineering Map

> **Purpose:** one canonical map of Sherlock's product domains, shared foundations, delivery priorities, and acceptance rules. This is a decision and execution map, not a work diary.
>
> **Reviewed:** 2026-11-24  
> **Repository:** `Aloews/Sherlock` — Rust-first rebuild  
> **Legacy source repository:** `Aloews/sherlock-scholes-`  
> **Detailed Economic Dynasty specification:** [`docs/ECONOMIC_DYNASTY.md`](ECONOMIC_DYNASTY.md)

## 1. Mission

Sherlock is a multi-domain product and simulation platform. Its first product domain is football: play, discover, understand, and simulate football worlds. The planned expansion is **Economic Dynasty**: explore how history, institutions, technologies, markets, businesses, and family decisions shape wealth and the choices available to later generations.

Sherlock must help users understand systems through reliable data, coherent simulations, and clear cause-and-effect exploration. It must not turn uncertain interpretations into facts or present a simulation as a forecast of what must happen.

### Product outcomes

1. **Play:** complete football game and career journeys reliably.
2. **Discover:** explore football entities, historical events, companies, institutions, markets, and people with traceable sources.
3. **Understand:** see definitions, timelines, relationships, and the limits of available evidence.
4. **Simulate:** make decisions in a consistent world with transparent rules, durable state, and explainable consequences.
5. **Learn across generations:** compare how family, business, and institutional decisions transmit advantages, risks, knowledge, and losses over time.

A screen is not a completed feature. Completion requires an end-to-end user path, trustworthy data, explicit failure states, relevant tests, and measured runtime evidence.

## 2. Current state and boundaries

The current `main` branch is a separate Rust-first rebuild. It is not the legacy production application.

Current repository evidence describes:
- an Axum API with `GET /healthz`;
- a Leptos WebAssembly client;
- a local two-team Alias game engine;
- a local fictional Football Dynasty career mode with a generated squad, training, transfers, a deterministic league, finances, heirs, and browser-local save data;
- a bundled starter catalogue of 2,145 football cards.

Current limitations remain release blockers, not optional footnotes: Alias is local-only; Football Dynasty is local-only; there is no verified cloud synchronization, Telegram identity integration, multiplayer synchronization, live Supabase card reads, or full legacy feature parity. See [`docs/DYNASTY_STATUS.md`](DYNASTY_STATUS.md) and [`docs/MIGRATION_SCOPE.md`](MIGRATION_SCOPE.md).

**Economic Dynasty is a planned product domain. It is not implemented by the current football career mode, is not a live financial data service, and must not be represented as shipped functionality.**

## 3. Product domain map

| Domain | User outcome | Core concepts | Current position |
|---|---|---|---|
| Football Alias | Play a complete word/football knowledge game | Rooms, teams, rounds, categories, cards, scoring | Local slice exists; multiplayer parity remains open |
| Football knowledge | Discover real players, clubs, matches, competitions, statistics, forecasts, and news | Canonical identities, sourced records, fixtures, metrics, predictions | Migration scope; verify each feature independently |
| Football Dynasty | Manage a fictional football club and pass control to heirs | Squad, tactics, training, contracts, transfers, club ledger, seasons, succession | Local playable slice; full lifecycle and cloud persistence remain open |
| Economic history | Explore how monetary, credit, political, technological, and production systems changed over time | Historical events, institutions, actors, sources, timelines, relationships | Planned |
| Markets and macroeconomics | Understand current market movements and how they transmit into the real economy | Rates, yields, currencies, equities, commodities, inflation, employment, credit | Planned; no live feed is claimed |
| Financial memory | Study how societies preserve or lose knowledge of previous crises | Archives, expertise, laws, procedures, education, institutional continuity | Central research hypothesis |
| Business and family dynasty | Simulate company ownership, capital allocation, debt, risk, succession, and intergenerational consequences | Firms, families, assets, liabilities, contracts, transactions, ledgers, inheritance | Planned |
| Cross-domain simulation | Reuse proven simulation mechanics without mixing domain facts | Actors, time, actions, reactions, events, state transitions, audit history | Shared foundations to be identified carefully |

### Domain boundary rule

Football, economic history, live markets, fictional economic scenarios, and user-created family/business campaigns are distinct bounded contexts. They may share genuinely common technical primitives, but must not share incompatible domain entities or silently reuse football semantics for economics.

## 4. Economic Dynasty — product thesis

The working title is **Economic Dynasty: the history of financial power and memory**.

The core question is not simply why bubbles happen. It is:

> **How do civilizations, institutions, companies, and families preserve financial knowledge across generations — and what changes when the carriers of that memory disappear?**

A second question connects history to the present:

> **How does a political, technological, monetary, or market event travel through companies and households, changing income, debt, asset values, opportunity, and the options available to the next generation?**

The research must test these hypotheses, not assume they are true. A dynasty can lose knowledge despite surviving; an institution can preserve knowledge without a ruling family; similar crises can have different causes and outcomes.

The full product contract is in [`docs/ECONOMIC_DYNASTY.md`](ECONOMIC_DYNASTY.md).

## 5. The analytical model

Every case study and current event should be explored through connected layers:

1. **Geography:** resources, climate, ports, trade routes, chokepoints, settlement, and distance.
2. **Production:** what creates real output and income in the period — for example grain, metals, textiles, ships, coal, oil, machinery, chips, or services.
3. **Technology:** changes in production, energy, transport, communications, computation, and financial infrastructure.
4. **Money and credit:** monetary forms, debt, collateral, interest, liquidity, payment systems, and state finance.
5. **Power and institutions:** rulers, states, banks, trading houses, firms, regulators, courts, religious institutions, universities, and central banks.
6. **Ownership and distribution:** who controls capital, who receives income, who carries liabilities, who bears losses, and who gets support.
7. **Narratives:** stories that coordinate expectations — for example a promised new era, a supposedly risk-free asset, or a claim that a technology removes old constraints. Study incentives and evidence; do not infer conspiracy from benefit alone.
8. **Generations and memory:** who experienced the event, who made decisions, who inherited the consequences, and which knowledge survived.

### Standard crisis/event record

For each historical crisis or modern market event, capture:

- period, date precision, geography, and affected systems;
- structural conditions and vulnerabilities before the event;
- production, technology, monetary, and credit context;
- actors, decision rights, incentives, and competing interests;
- trigger, transmission channels, and documented responses;
- winners, losers, transfers, bailouts, defaults, and public costs where evidence permits;
- reforms and new institutions that followed;
- carriers of financial memory and evidence of continuity or loss;
- generational exposure and behavioural evidence, where measurable;
- a carefully qualified connection to present-day events;
- sources, citations, confidence, unresolved disputes, and data limitations.

Do not collapse structural causes, triggers, accelerants, and consequences into one label.

### Relationship vocabulary

The historical knowledge graph must distinguish:
- `caused_by` — use only when the evidence supports a causal claim;
- `contributed_to` / `amplified_by` — a documented contributing mechanism;
- `preceded_by` — chronology, not causation;
- `correlated_with` — co-movement without a causal assertion;
- `responded_to` — policy or institutional response;
- `transferred_loss_to` / `benefited` — evidenced distributional effects;
- `institutionalized_as` — a practice or lesson encoded in law, policy, or process;
- `preserved_by` / `transmitted_through` / `lost_after` — claims about memory continuity that require evidence.

Never use a visually compelling graph edge to imply more certainty than the underlying source supports.

## 6. Financial memory and generations

Treat **financial memory** as an empirical research question. Potential carriers include families, dynasties, banks, trading houses, bureaucracies, courts, religious institutions, universities, professional associations, archives, and central banks.

Examine at least five dimensions separately:
- archive and record continuity;
- continuity of personnel and professional expertise;
- continuity of laws, regulations, and operating procedures;
- transmission through education, apprenticeship, and professional culture;
- continuity of incentives and institutional governance.

Do not create a single numerical “memory score” until defensible measures, weights, and validation exist. Show missing evidence rather than filling it with an invented score.

### Generational analysis

Do not assert that a particular age group naturally creates bubbles or behaves in one fixed way. Distinguish:
- birth cohort and formative macroeconomic experience;
- age and life stage at the time of the event;
- role in the system (founder, worker, lender, owner, regulator, executive, heir, investor);
- actual control over capital and decisions;
- wealth exposure, leverage, and ability to absorb losses;
- what was transmitted to descendants or successors.

For ancient periods, use documented social roles and succession positions rather than pretending that reliable modern age statistics exist. For modern periods, use cohort evidence only with sample size, methodology, and alternative explanations.

## 7. History coverage and candidate case studies

The intended timeline begins with early debt and state finance and continues into current digital markets. These are **research candidates, not pre-approved conclusions**:

| Period / case | Questions to investigate |
|---|---|
| Ancient Mesopotamia and other early states | Credit, collateral, debt dependence, debt remission, temple/palace production, and political stability |
| Archaic Greece and Rome | Debt reform, land and credit markets, elite status, liquidity, and state intervention |
| Medieval Italian city-states | Merchant banking, sovereign lending, trade networks, defaults, and concentration risk |
| Yuan China and other monetary transitions | Paper money, fiscal pressure, military spending, monetary credibility, and institutional knowledge |
| Dutch Republic and tulip trade | Contract forms, social reach, luxury/status narratives, market depth, and limits of the “mania” story |
| Mississippi and South Sea schemes | Sovereign debt conversion, equity, monetary expansion, political influence, and mass participation |
| Railway Mania and industrialization | Infrastructure finance, leverage, real productive value, overbuilding, and later productivity gains |
| 1907, 1929, and the Great Depression | Liquidity, leverage, bank failures, central banking, regulation, and distribution of losses |
| Bretton Woods and 1971 | Monetary order, reserve currency, convertibility, geopolitical power, and policy constraints |
| 1970s inflation and Volcker disinflation | Energy shocks, expectations, wages, interest rates, unemployment, and intergenerational effects |
| 1987, 1997, 1998, and dot-com | Market structure, currency mismatch, leverage, technology narratives, and policy backstops |
| 2008 and the euro-area debt crisis | Housing credit, securitization, bank-sovereign links, bailouts, austerity, and regulatory change |
| 2020–2023 | Pandemic policy, supply shocks, inflation, duration risk, bank runs, and digital investor participation |
| 2024–present | AI investment, compute and energy demand, valuations, productivity evidence, and concentration risk |

Select cases for evidentiary value and comparability, not just fame. Include counterexamples where an institution preserved knowledge or a dynasty failed to do so.

## 8. Current markets and economic dashboard

The planned dashboard must be simple at first glance and analytically deep on demand.

### Top-level panels

- policy rates and central-bank decisions;
- sovereign yields and yield-curve changes;
- major currencies and exchange-rate changes;
- equity indices and sector/region breadth;
- commodities, especially energy and monetary metals;
- inflation, growth, employment, and business surveys;
- credit conditions, spreads, defaults, and liquidity indicators where reliable data exist;
- a dated macro/event calendar.

### Every observation must carry

- value, unit, currency, geography, and instrument/indicator identity;
- observation time, timezone, and market/session status;
- source, original publication time, and retrieval time;
- period being measured and whether the value is preliminary, revised, estimated, or final;
- comparison baseline (previous close, previous release, consensus, or another explicit reference);
- missing/stale/conflicting-data state.

Never compare values with incompatible units, currencies, periods, or market sessions. Never render an intraday quote as a confirmed close. Do not invent consensus when no supported consensus source exists.

### Event-to-household chain

Each important news item should explain the plausible transmission path:

**event → rates / prices / FX / credit / supply chains → sector and company exposure → wages, employment, borrowing costs, savings, and asset values → family choices and successor opportunities.**

Label each link as observed fact, sourced estimate, analytical mechanism, or scenario. A plausible channel is not proof that a specific household or company was affected.

## 9. Business and family dynasty simulation

The future simulation should let a player manage a family and one or more businesses over multiple generations. Potential decisions include investment, borrowing, saving, production, hiring, pricing, innovation, risk management, ownership, philanthropy, succession, and resilience during shocks.

Core records should support:
- family members, relationships, succession rules, roles, and decision authority;
- companies, ownership shares, governance, employees, suppliers, and customers;
- assets, liabilities, cash flows, revenue, costs, wages, taxes, and debt service;
- contracts, loans, collateral, transactions, and audit history;
- market and policy environments at each simulated date;
- shocks, decisions, actions, reactions, and delayed consequences;
- inheritance, transfer of control, and continuity of institutional knowledge.

Simulation results must be reproducible from the same versioned rules, initial state, random seed, and event sequence. Financial calculations require explicit currency and precision; do not use binary floating-point as the source of truth for money. Maintain auditable ledgers and reconcile opening balance, movements, and closing balance.

Keep three worlds distinct:
1. observed historical/current facts;
2. fictional but historically calibrated scenarios;
3. user-controlled simulation state.

Never write simulation output back into real-world datasets.

## 10. Football as a reference implementation for business mechanics

The existing Football Dynasty is a useful reference for reusable simulation patterns, not a domain model to copy blindly.

| Football simulation concept | Possible business/economic analogue |
|---|---|
| Club ownership | Company or holding ownership |
| Squad and staff | Employees, managers, and specialist teams |
| Player development | Workforce capability and human capital |
| Transfer / contract negotiation | Hiring, acquisition, sale, and commercial contracts |
| Season and fixtures | Fiscal periods, market cycles, and scheduled obligations |
| Club cash ledger | Company cash-flow and accounting ledger |
| Rival clubs | Competitors and market participants |
| Heir and succession | Family ownership and leadership succession |
| Match/event outcomes | Business decisions and market outcomes |

The analogy is a design aid only. Football-specific rules, player statistics, club tables, and fictional football entities must never leak into economic records. Shared code is justified only where invariants are genuinely identical and covered by domain-specific tests.

## 11. Architecture principles

- Keep domain logic in Rust and keep the client as a presentation/input layer.
- Preserve bounded contexts: football, historical knowledge, macro/markets, and business/family simulation.
- Share infrastructure only where it is truly generic: typed identifiers, time handling, provenance, audit events, authorization, localization, and tested UI primitives.
- Use explicit contracts for historical events, sources, market observations, financial transactions, and simulation commands.
- Version source definitions, calculation rules, simulation rules, and model outputs.
- Keep ingestion, normalization, domain logic, API, and UI independently testable.
- Use server-side authorization and ownership checks for persistent user campaigns.
- Prefer additive migrations, idempotent jobs, replayable commands, and auditable state transitions.
- Do not select a database schema, market vendor, or new crate layout until source requirements, access terms, operational costs, and data retention needs have been checked.

## 12. Experience and visual direction

The experience should feel like a premium analytical product, not a spreadsheet wall or a trading terminal designed to provoke impulsive decisions.

- Show the most important state first; reveal causal detail progressively.
- Use plain-language explanations for unfamiliar economic terms.
- Make source, timestamp, uncertainty, and data freshness accessible without overwhelming the main view.
- Use timelines and relationship views where they clarify time and causality; never use decorative connections that imply unsupported causes.
- Give users a clear path from headline → indicator → affected sector → business/family exposure → historical comparison.
- Distinguish observed values, estimates, forecasts, and simulations visually and consistently.
- Design mobile and desktop layouts deliberately; do not merely shrink a dense desktop table.
- Avoid noisy charts, false precision, unverified badges, and unnecessary explanatory copy on action controls.

## 13. Delivery roadmap and priority

### P0 — Finish and verify the core Rust football product

Continue the currently documented migration and Football Dynasty acceptance work. Do not claim full parity or production readiness based on a compile or a local browser save. See `docs/MIGRATION_SCOPE.md` and `docs/DYNASTY_STATUS.md`.

### E0 — Economic Dynasty specification and evidence foundation

- Keep this map and `docs/ECONOMIC_DYNASTY.md` as the product contract.
- Build a source registry and evidence rubric before collecting a large historical corpus.
- Define event, actor, institution, relationship, financial observation, and family/business domain contracts.
- Select a small pilot set of contrasting historical cases and document competing interpretations.
- Record research questions and disconfirming evidence, not just supporting examples.

### E1 — Historical timeline and knowledge graph

- Deliver a source-linked timeline and event detail journey.
- Implement typed, testable relationship semantics and confidence/provenance display.
- Prove that users can distinguish chronology, correlation, contribution, and causation.
- Evaluate at least one counterexample to the financial-memory hypothesis.

### E2 — Current markets dashboard

- Connect a small, well-defined set of trusted data sources.
- Show freshness, units, observation windows, revisions, and unavailable states.
- Produce concise daily event-to-economy explanations with links to underlying sources.
- Validate calculations against source values and independent checks.

### E3 — Business and family dynasty vertical slice

- Build one reproducible company/family campaign with a documented time step.
- Implement ledger integrity, one financing decision, one operating decision, a shock, and a succession event.
- Verify deterministic replay, ownership isolation, persistence, and recovery.
- Calibrate only to evidence that is appropriate for the scenario and show assumptions.

### E4 — Generational and institutional memory systems

- Model different carriers of memory without privileging family dynasties by assumption.
- Test how records, personnel, rules, and education affect later decisions.
- Add scenario comparison and counterfactuals only when assumptions are explicit and results are reproducible.

### E5 — Integrated Sherlock experience

Connect the new domain to shared navigation and platform capabilities only after its own data, privacy, security, and quality gates pass. Do not delay core football release work by prematurely adding a placeholder Economic Dynasty screen.

## 14. AI-agent operating contract

Every agent working on Sherlock must:
1. Read this map, `docs/MIGRATION_SCOPE.md`, `docs/ECONOMIC_DYNASTY.md` when relevant, and the feature-specific status/spec before changing code.
2. Inspect the actual branch, working tree, linked issues/PRs, relevant source code, tests, and CI evidence before planning.
3. Translate a request into one bounded vertical slice with measurable acceptance criteria.
4. Keep the research/evidence role separate from the implementation role: code generation is not historical verification.
5. Use independent adversarial review for data provenance, causal claims, financial arithmetic, authorization, and user-facing interpretation.
6. Never turn a research hypothesis into a product fact, a market quote into a forecast, or a simulated value into a real observation.
7. Do not introduce a new pipeline, duplicate source of truth, or parallel domain engine without an explicit architecture decision.
8. Run targeted tests, full required repository checks, and relevant browser/runtime journeys; inspect exact-head results and logs.
9. Update the map when a durable architecture decision, product boundary, priority, or acceptance gate changes. Put incident logs and raw research notes in dedicated reports, not in the map.
10. Report implemented, tested, runtime-verified, and production-deployed states separately.

### Agent specialties to combine for Economic Dynasty

- **Engineering lead:** scope, integration, architecture decisions, and evidence-backed completion.
- **Data integrity / research:** source quality, temporal validity, provenance, conflicting interpretations, and reproducible data.
- **Rust backend:** typed domain model, financial precision, deterministic simulation, persistence, and transaction integrity.
- **Frontend product QA:** information hierarchy, accessible visualization, responsive flows, and plain-language comprehension.
- **Adversarial QA:** challenge assumptions, causal claims, model leakage, edge cases, and scenario reproducibility.
- **Security reviewer:** authorization, personal financial data boundaries, auditability, and safe integrations.

Use actual specialist agents only when the runtime exposes them. Otherwise work sequentially and do not claim parallel-agent execution.

## 15. Definition of done

A domain feature is complete only when:
- [ ] Its user outcome and scope are explicit.
- [ ] Its facts, estimates, assumptions, and unknowns are separated.
- [ ] Source provenance and timestamps are available where applicable.
- [ ] Financial, temporal, and ownership invariants are encoded and tested.
- [ ] Success, failure, boundary, stale-data, and recovery cases are tested.
- [ ] The exact candidate commit passes relevant CI.
- [ ] The real user journey is smoke-tested in the target runtime.
- [ ] Data correctness, freshness, simulation determinism, and user impact are measured where relevant.
- [ ] Documentation reflects shipped behavior, not future intent.

**Priority rule:** complete and verify one useful vertical slice before expanding breadth. A green build is necessary but is not proof of product acceptance.
