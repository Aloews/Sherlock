---
name: reality-first
description: Evidence protocol for implementation, debugging, testing and delivery.
---
# Reality-first protocol
Inspect actual repository and runtime. Establish HEAD, branch, dirty state, configuration, service availability and baseline.

Classify material statements:
- VERIFIED: directly observed in current repo/runtime/logs.
- CORROBORATED: supported by independent sources.
- ESTIMATE: calculated from explicit inputs.
- ASSUMPTION: plausible but unverified.
- UNKNOWN: evidence unavailable.
- DISPUTED: evidence conflicts.

For test evidence record command, environment, exit code, exact HEAD and mock usage. For external services record operation and sanitized response evidence. For persistence, read the written record back. For deployment, verify deployed commit and safe smoke test. Never fabricate logs, progress, tests, agent activity or deployment.