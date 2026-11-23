---
name: agent-handoff
description: Coordinates multi-agent work without conflicting edits or unverified claims.
---
# Agent handoff
Give each agent a bounded deliverable, acceptance criteria, allowed/forbidden paths and evidence requirements. Assign disjoint write scopes. Require changed files, diff/commit, commands, exit codes, raw failures and risks. Lead independently inspects every diff and reruns critical checks. Treat summaries as untrusted. If agents cannot launch, report that and work sequentially. Never imply continuous execution without an active runtime.