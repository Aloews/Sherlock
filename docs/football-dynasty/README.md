# Football Dynasty

Football Dynasty is a long-running strategy simulation inside Sherlock. The family dynasty, not one manager's career, is the unit of progression. The design target combines football operations, family succession, finance, rival houses, politics and a persistent causal world.

## Included backend

The separate Rust service in `services/football-dynasty` contains:
- campaign, character, family and succession lifecycle;
- fictional club ownership and balanced opening/journal transactions;
- a bounded campaign calendar and persisted decision inbox;
- fictional competitions, fixtures, deterministic match results, standings and match reports;
- match rosters, goals, substitutions, spatial events and player-match records;
- persistent world incidents, actor knowledge, player responses and delayed reactions;
- PostgreSQL migrations, security policies and PostgreSQL-backed integration tests.

Real-world football catalogue data and the Football Alias game are separate domains. Dynasty data is fictional and isolated in the private `fd` schema.

Read [GDD](GDD.md), [Architecture](ARCHITECTURE.md), [Data Model](DATA_MODEL.md), [Living World](LIVING_WORLD.md), and [Implementation Plan](IMPLEMENTATION_PLAN.md).