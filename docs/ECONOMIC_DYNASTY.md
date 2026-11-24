# Economic Dynasty — Product and Domain Specification

> **Status:** planned product domain; specification only. No current implementation, market feed, historical database, or personal-finance integration is implied.
>
> **Parent map:** [`docs/MAP.md`](MAP.md)  
> **Migration and agent-planner summary:** [`docs/MIGRATION_SCOPE.md`](MIGRATION_SCOPE.md)  
> **Reviewed:** 2026-11-24

## 1. Product purpose

Economic Dynasty is a research-led historical and business simulation inside Sherlock. It connects:
- historical events and monetary systems;
- geography, production, and technological change;
- political power, institutions, and financial markets;
- companies, trade networks, and capital ownership;
- families, generations, and transmission of financial knowledge;
- current macroeconomic news and its plausible effects on firms and households.

The product should let a user move from a present-day headline to the underlying market channel, affected industry, company/family exposure, historical comparison, and possible long-term consequence.

This is an educational and analytical product. It is not an automated investment adviser, broker, trading system, or promise of market prediction.

## 2. Research hypotheses

### H1 — Financial memory is distributed

Societies preserve knowledge of financial crises through multiple carriers: families, trading houses, banks, state bureaucracies, courts, legal rules, professional communities, universities, archives, and central banks. A family dynasty is one possible carrier, not the assumed master explanation.

### H2 — Memory continuity may alter later crisis responses

A crisis can be followed by archives, procedures, laws, education, new regulators, and professional practices that change later responses. Test whether and how this continuity is associated with different outcomes.

### H3 — Memory can decay or fail

Knowledge may disappear when institutions collapse, personnel disperse, records are lost, incentives change, or later generations stop treating prior warnings as relevant. Identify observable mechanisms; do not infer forgetting merely because a similar crisis recurs.

### H4 — Financial crises are intergenerational

People experience the same event from different positions: borrower, worker, owner, lender, regulator, executive, policymaker, heir, or later entrant. Experience can influence later risk-taking, but age and cohort effects must be distinguished from wealth, role, incentives, institutions, and calendar period.

### H5 — Narratives coordinate expectations

Stories about a new era, a revolutionary technology, an asset that cannot fall, or a new monetary order can coordinate investment and political behaviour. Analyse who creates and distributes a narrative, who benefits, how widely it spreads, and whether evidence supports the promised outcome. Do not equate influence or benefit with deliberate orchestration.

Every hypothesis needs supporting evidence, counterexamples, competing explanations, and explicit confidence. Hypotheses remain hypotheses until research supports a narrower claim.

## 3. Research method and evidence rules

### Evidence hierarchy

Prefer sources in this order, while evaluating the specific document rather than relying on the source's name alone:

1. Primary historical records, statutes, court records, official accounts, central-bank publications, statistical offices, company filings, and original datasets.
2. Peer-reviewed historical/economic research and academic books from reputable publishers.
3. Working papers from established research institutions, with working-paper status clearly stated.
4. Reputable journalism for chronology, contemporary reporting, and interviews.
5. Secondary explainers only as discovery aids, not sole support for contested claims.

For every claim retain source title, author/institution, publication date, URL/identifier, page/table/section when available, access date, and the exact claim supported. Record whether the source is primary, peer-reviewed, working paper, journalism, or a secondary summary.

### Claim labels

Use explicit labels in research records and the UI:
- **Verified fact:** directly supported by a reliable source or primary record.
- **Corroborated claim:** supported by multiple independent reliable sources.
- **Estimate:** a quantitative approximation with method and range.
- **Interpretation:** an author's or analyst's explanation, not an established fact.
- **Hypothesis:** a proposition to test.
- **Disputed:** credible sources disagree.
- **Unknown / unavailable:** current evidence does not resolve the question.

Record contradictory evidence. Do not silently choose the most vivid version.

### Causal discipline

Distinguish:
- long-term structural conditions;
- proximate causes;
- trigger events;
- amplifying feedback loops;
- policy responses;
- distributional effects;
- later institutional consequences.

Chronological order is not causation. Correlation is not causation. A beneficiary is not automatically an orchestrator. Avoid modern labels such as “central bank bailout”, “mortgage bubble”, or “leverage” for ancient events unless the analogy is explicitly explained and qualified.

### Counterexample requirement

Each major thesis needs at least one deliberate counterexample search. Examples:
- a long-lived dynasty that failed to retain financial knowledge;
- a non-dynastic institution that preserved expertise across political change;
- a crisis followed by repeated losses despite formal reform;
- an apparent bubble where the popular story overstates participation or misidentifies the mechanism.

## 4. Historical coverage

Start with a selected pilot, not a massive low-quality scrape. Candidate cases include:

- Mesopotamian credit, debt dependence, and debt remissions;
- Solonian debt reforms and the Near Eastern comparisons;
- Roman credit and land-market stress, including the events of 33 CE;
- Bardi and Peruzzi merchant banking and sovereign lending;
- Dutch Republic tulip contracts and the limits of the classic mania narrative;
- John Law and the Mississippi scheme; the South Sea Bubble;
- British Railway Mania and infrastructure finance;
- the 1907 US panic and creation of the Federal Reserve;
- 1929 and the banking crises of the Great Depression;
- Bretton Woods, the 1971 end of dollar-gold convertibility, and the 1970s inflation;
- 1987, the Asian crisis, Russia/LTCM, and dot-com;
- 2008 global financial crisis and the euro-area sovereign-debt crisis;
- COVID-era policy, inflation, the 2023 regional US bank failures;
- current AI/compute investment, energy constraints, valuations, and productivity evidence.

For early periods, analyse roles and institutions rather than inventing age-cohort statistics. For modern periods, preserve definitions, time windows, sample selection, and uncertainty.

## 5. Economic event and knowledge-graph contracts

An event record should be able to represent:
- stable event ID and version;
- start/end dates with date precision and calendar convention;
- location and geographic scope;
- actors and institutions with time-valid relationships;
- event category and affected sectors;
- preconditions, vulnerabilities, triggers, mechanisms, and responses as separate records;
- monetary, production, technology, trade, and political context;
- observed effects and distribution of gains/losses;
- resulting rules, organisations, and practices;
- related families, firms, cohorts, and later events;
- sources, citations, confidence, disputed fields, and missing coverage.

Use typed relationships. At minimum distinguish causation, contribution, amplification, chronology, correlation, response, ownership, credit exposure, benefit, loss, institutionalization, preservation, transmission, and discontinuity.

Relationships should be temporal: ownership, offices, legal status, alliances, and institutional control can change. Do not treat a relationship documented for one year as permanent.

## 6. Financial-memory model

Represent memory carriers and evidence separately.

Potential carrier types:
- family or ruling house;
- private bank, merchant bank, or trading house;
- state treasury, bureaucracy, regulator, or central bank;
- court, legal institution, or professional association;
- religious institution, school, university, or archive;
- company, industry association, or international organization.

Potential continuity observations:
- original records survive and remain accessible;
- staff or professional lineages continue;
- procedures are documented and reused;
- laws or regulations encode lessons;
- knowledge is taught to successors or new entrants;
- later policy explicitly cites or adapts earlier experience.

Do not turn this into a synthetic score until a transparent measurement model is validated. A missing archive is not proof that no knowledge survived; absence of evidence must be represented carefully.

## 7. Family, firm, and generation model

### Family domain

Potential records: person, family relationship, cohort, life stage, role, ownership share, control rights, inheritance rule, succession event, decision record, and transmitted knowledge.

### Business domain

Potential records: company, beneficial owner, shareholding, governance role, employee, supplier, customer, asset, liability, contract, loan, collateral, transaction, cash flow, tax, wage, and insolvency event.

### Generation and exposure

For a person or cohort, capture where evidence permits:
- age or life stage at event;
- formative events and duration of exposure;
- socioeconomic position and wealth;
- role and decision rights;
- debt, asset, employment, and geographic exposure;
- later behaviour and available comparison group;
- uncertainty and confounding variables.

Never assign fixed financial personalities to generations. A cohort-level pattern is not a prediction about an individual.

## 8. Current market intelligence dashboard

### Initial scope

Start with a small sourceable panel rather than every possible indicator:
- policy rates and central-bank decisions;
- government bond yields and curve changes;
- broad equity indices and sector breadth;
- major foreign-exchange rates;
- gold and key energy/industrial commodities;
- inflation, growth, employment, and selected business surveys;
- credit spreads/default measures when consistent data is available;
- near-term macroeconomic and central-bank calendar.

### Observation contract

Each value requires:
- instrument/indicator identifier, geography, value, currency/unit;
- observation period and exact timestamp/timezone;
- source and direct link, publication time, retrieval time;
- previous/consensus/revised value only when sourced and comparable;
- quote/session status for market prices;
- quality status: current, stale, preliminary, revised, conflicting, or unavailable.

Do not merge futures, spot, and index values as though they are the same instrument. Do not label intraday values as closing prices. Do not infer market direction from a headline without checking the underlying data.

### Daily briefing format

1. Market snapshot with time and units.
2. Three to five material events, each linked to an underlying source.
3. What happened versus what was expected, when expectations are available.
4. Transmission mechanism: rates, FX, costs, demand, financing, sector exposure, households.
5. Relevant historical analogy, including differences and counter-evidence.
6. Scenario/watch list for upcoming data and policy events.
7. Explicit unknowns and source gaps.

Ordinary daily changes are still reported as ordinary changes; no dramatic language is needed. Every figure should be reproducible from its source and retrieval timestamp.

## 9. Company and family simulation

The first playable vertical slice should be deliberately small:
- one family with a founder, successor, and clear ownership/control rules;
- one company with a simple product, workforce, revenue, operating costs, assets, and liabilities;
- a dated economic environment with interest rates, demand, input costs, and one external shock;
- a few decisions: borrow/save, invest/defer, hire/retain, price/produce, diversify/focus;
- a balanced transaction ledger;
- one succession or ownership transfer;
- a multi-period report showing company health and family wealth separately.

A simulation should communicate:
- what the player controlled;
- what the environment changed;
- which rule produced each result;
- which assumptions are scenario inputs;
- which effects are uncertain or random;
- what is inherited by successors.

Use fixed-point or decimal-safe money arithmetic, explicit currencies and exchange-rate dates, double-entry or otherwise fully reconcilable ledgers, versioned rules, and deterministic replay. Do not use live market values as automatic game outcomes without an explicit model and user-facing uncertainty.

## 10. Relationship to Football Dynasty

Reuse only validated generic capabilities such as:
- simulation clock and period progression;
- actors, organisations, roles, and ownership;
- decision/action/reaction event history;
- contracts and transactions;
- auditable ledgers;
- succession and transfer of control;
- deterministic scenario replay;
- save/restore and authorization infrastructure.

Do not reuse football-specific entities or business rules. A player transfer is not literally a company acquisition; a football season is not literally a business cycle. Shared abstractions must be justified by identical invariants and tested independently in each domain.

## 11. Technical and security boundaries

- Rust owns authoritative business, financial, timeline, and simulation rules.
- Frontend components render typed state and collect decisions; they do not recalculate balances or simulate outcomes independently.
- Store all times with explicit timezone/precision rules; store money with currency and decimal-safe precision.
- Maintain data lineage from external source through ingestion, normalization, storage, API, and UI.
- Separate historical facts, live observations, analyst estimates, forecasts, and simulation output at schema and API levels.
- Keep user-created family/business campaigns owner-scoped with server-side authorization.
- Do not connect bank accounts, brokerages, or ingest personal financial records in the initial scope.
- Do not execute trades, move money, or provide personalized buy/sell instructions.
- Review licensing, terms, rate limits, and retention before adding data vendors or scraping.
- Never put API keys, secrets, private user data, or production credentials in this public repository.

## 12. Phased acceptance gates

### ED-0 — Research foundation
- Source registry and claim taxonomy exist.
- Initial pilot cases have primary/academic sources and documented disputes.
- Each case contains a counterexample or explicit record that the search remains open.
- The financial-memory hypothesis is expressed as testable subquestions.

### ED-1 — Historical timeline and graph
- Events are versioned and linked to source passages.
- Timeline ordering respects date precision and uncertain ranges.
- Graph views distinguish causation, correlation, chronology, and response.
- A user can inspect why an edge is asserted and which sources support it.

### ED-2 — Market dashboard
- All displayed observations show units, timestamps, source and session/period.
- Reconciliation tests compare imported values to source records.
- Stale and missing data are visible and do not silently become zero.
- Daily briefing claims resolve to sourced event/indicator records.

### ED-3 — Business/family simulation
- Scenario replay is deterministic for a fixed version, seed, and command sequence.
- Opening balance + movements = closing balance in every ledger test.
- Currency, rounding, debt service, ownership, inheritance, and date boundaries have focused tests.
- Create → save → reload → decision → shock → succession → restore journeys pass.

### ED-4 — Intergenerational/institutional memory
- Different memory carriers can be compared without assuming family superiority.
- Cohort-level findings disclose source, methodology, sample, and confounders.
- Counterfactuals are labelled as scenarios and expose their assumptions.

### ED-5 — Sherlock integration
- Navigation, access control, persistence, responsive UI, observability, and recovery are verified.
- No football or legacy production data is modified by the new domain.
- The feature passes exact-head CI and real browser journeys before release.

## 13. Implementation rules for agents

- Read this specification and the parent project map before proposing code.
- Begin with a bounded, testable vertical slice; do not build a giant universal economic engine first.
- Prefer source registry, schemas, and typed contracts before large ingestion jobs or polished visualizations.
- For each research feature, ask what observation would falsify the intended explanation.
- For each numeric feature, write units, time basis, rounding, provenance, and reconciliation tests before UI work.
- For each simulation, version rules and prove deterministic replay and ledger integrity.
- Never substitute generated historical facts, invented quotations, guessed market values, or placeholder source URLs for evidence.
- Do not implement a live data provider without checking API availability, permissions, rate limits, costs, and a failure/degradation path.
- Do not create placeholder controls that appear operational.
- Keep current football parity work ahead of Economic Dynasty implementation unless the product owner explicitly reprioritizes it.
- Report the feature state accurately: specified, implemented, tested, runtime-verified, or production-released.

