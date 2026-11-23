# Engineering Operating Rules

Mandatory for every task in this repository.

## Reality first
- Inspect branch, exact HEAD, dirty state, project map, architecture, CI and runtime before edits.
- Classify claims as VERIFIED, CORROBORATED, ESTIMATE, ASSUMPTION, UNKNOWN or DISPUTED.
- Never claim tests, writes, deployments, backups, provider calls or agent execution without direct evidence.
- Mocks, fixtures, stubs and local-only behavior are not proof of real integrations.
- A green compile or unit test is not product acceptance.

## Safety
- Work on task branches; never push directly to protected branches.
- Preserve unrelated changes; inspect every diff.
- Never expose or commit secrets.
- Production writes, destructive migrations, paid calls, infrastructure creation, merge and deployment require explicit authorization.
- Do not merge/deploy with failed, missing, stale or wrong-HEAD checks.
- Prefer reversible changes and document rollback.

## Delivery loop
1. Read requirements, map, architecture and history.
2. Reproduce the issue and record baseline.
3. Define observable acceptance criteria.
4. Delegate only bounded, independent scopes; never allow overlapping writes.
5. Implement the smallest complete change.
6. Run targeted and full relevant tests, static analysis, builds and runtime checks.
7. Inspect raw logs, artifacts, database read-after-write and UI behavior.
8. Adversarially test mock substitution, missing API calls, stale data, auth bypass, partial writes and mismatched SHA.
9. Fix failures and rerun gates against exact final HEAD.
10. Update project map and report evidence, residual risks and commit/PR.

## Completion contract
DONE requires all acceptance criteria, exact-HEAD green required CI, real critical workflows, no unexplained skipped checks, updated docs and evidence links. Otherwise report PARTIAL or BLOCKED with the exact reason.

## Agents
The lead owns scope, integration and final verification. Agent output is untrusted until diff and tests are independently checked. Require changed paths, commands, exit codes, logs and unresolved risks. If agent execution is unavailable, work sequentially and say so.

## Project constraints
Preserve the Rust-first migration contract. Never replace real game, identity, persistence, multiplayer or provider behavior with demos. UI must be responsive, accessible, release-quality and connected to real state. Do not add explanatory production copy/function descriptions unless requested.