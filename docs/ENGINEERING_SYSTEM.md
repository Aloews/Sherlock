# Shared Engineering System

This is a reusable operating model, piloted in Sherlock. Portable files are under `.claude/`. Copy only after reviewing each target project's existing rules.

## Roles
- engineering-lead: scope, acceptance, coordination, integration and final evidence.
- rust-backend: Rust domain logic, APIs, concurrency, persistence and tests.
- qa-adversarial: independent attempts to falsify acceptance and test claims.
- security-reviewer: application, data, infrastructure and supply-chain security.
- frontend-product-qa: responsive, accessible UI and real user journeys.
- data-integrity: provenance, pipelines, schemas, analytics and persistence.

## Skills and commands
Skills: reality-first, release-gate, agent-handoff.
Commands: /verify, /finish-task, /security-gate.

## Sherlock adapter
Preserve Rust-first migration. Do not call the product production-ready until parity, auth, persistence, multiplayer/realtime, Telegram flows, deployment and rollback are proven. Local-only game behavior must not be described as cloud-synchronized.

## UHS adapter
Preserve security-first hub architecture, durable execution and recovery. Prove real provider/model calls, health discovery, failover, bounded tool execution, checkpoint restore, authorization and audit evidence. An API/mock response does not prove autonomous operation.

## Other project adapters
Add stack, data sources, critical user flows, production boundaries, acceptance tests and rollback. Keep common policy here; document exceptions explicitly.

## Adoption
1. Copy CLAUDE.md and .claude/ into the target repo, merging rather than blindly overwriting.
2. Add its project adapter.
3. Add stack-specific tests only after inspecting the repository.
4. Verify component discovery in the actual Claude Code version.
5. Run a dry-run without production credentials or paid services.

## Limits
These files do not install themselves into Claude Code, connect MCPs, create credentials, launch background agents or enable continuous execution. Each repo/workspace must adopt and verify them. No direct production writes, destructive migrations, paid calls, infrastructure creation, merge or deployment without explicit authorization.

 
## Runtime-specific roles
- `alias-game-engine`: deterministic Rust Alias rules, timer, scoring and match completion.
- `dynasty-simulation`: Football Dynasty simulation, transfers, finances, progression and succession.
- `provider-integration`: external service adapters, protocol correctness and failure handling.

## Structural validation
Run `python3 scripts/validate_agent_system.py`. The `Validate agent system` GitHub Actions workflow runs the same check on relevant pull requests and pushes. It validates file layout, metadata and final newlines only. It does not prove that Claude Code loaded the agents or that subagents executed.
