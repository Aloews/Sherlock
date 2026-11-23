---
name: release-gate
description: Mandatory pre-merge and pre-release acceptance gates.
---
# Release gate
1. Map each requirement to observable acceptance.
2. Inspect diff, generated files, dependencies and secrets.
3. Run format, lint, typecheck, unit, integration, migration and build checks.
4. Launch actual runtime and exercise critical workflows with intended dependencies.
5. Verify authorization, persistence, read-after-write, idempotency and rollback.
6. Run applicable security and adversarial tests.
7. Verify responsive UI, real controls, accessibility and failure states.
8. Require all checks green on exact final HEAD; stale runs do not count.
9. Document rollback/recovery for risky changes.
10. Attach commit, run IDs, logs, commands and residual risks.
Missing or failed critical gate means BLOCKED, never DONE.