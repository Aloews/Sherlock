# Football Dynasty — Data Model

The private PostgreSQL `fd` schema and migrations under `services/football-dynasty/migrations` define the currently implemented model. The wider catalogue below remains the target for later stages.

| Domain | Responsibility |
|---|---|
| Campaigns and worlds | Owner, mode, seed, game clock, simulation version |
| Dynasties, characters, family relations | House identity, generations, kinship and succession |
| Clubs and ownership | Fictional clubs, equity and control |
| Accounts and journal entries | Balanced finance and immutable transaction history |
| Competitions, seasons and fixtures | Fictional competition calendar and results |
| Match rosters and match events | Line-ups, goals, substitutions and match reports |
| Player lifecycle | Player progression and persisted football careers |
| World events, facts, knowledge and reactions | Causal incidents and delayed consequences |
| Events, choices and chronicle | Player decisions and history |
| Command receipts | Idempotency and replay protection |

## Invariants

- Every campaign is isolated by owner and campaign ID.
- Client code cannot write balances, ownership, succession state or simulation output.
- Journal entries balance; corrections are compensating entries.
- Non-repeatable mutations require an idempotency key.
- Succession is atomic and cannot transfer the estate twice.
- Campaign time is monotonic and bounded per command.
- World events and chronicle history survive reloads.
- Fictional entities never enter the real-football catalogue.

The schema continues to evolve. A passing local/CI migration test does not prove that a production database has been provisioned or migrated.
