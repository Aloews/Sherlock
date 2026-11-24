# Rust agent cycle

## Purpose

A repository-owned coding loop for the Rust rewrite. It uses a local Ollama model on a GitHub-hosted runner and separates planning, implementation, and review into role-specific model calls.

## Canonical planning inputs

- Read `docs/MAP.md` for product intent, domain boundaries, priorities, and definition of done.
- Read `docs/MIGRATION_SCOPE.md` for the active migration scope and sequencing rules.
- For Economic Dynasty tasks, read `docs/ECONOMIC_DYNASTY.md` as the detailed research, data, simulation, and acceptance contract.
- The current autonomous task planner reads `docs/MIGRATION_SCOPE.md` directly when it derives a task after the seed backlog is exhausted. Therefore the concise Economic Dynasty scope, priority boundary, and spec link must remain summarized in `docs/MIGRATION_SCOPE.md`; do not assume the planner automatically reads every document.
- A task touching an established domain must also read that domain's status/spec and the relevant source/tests.

## Execution sequence

1. A successful run of **Rust CI** on the bootstrap commit activates the first agent iteration.
2. The planner selects one bounded task from `.github/agent-backlog.json`. Once the initial queue is exhausted, it derives one new task from `docs/MIGRATION_SCOPE.md`.
3. The implementer proposes a unified diff. A fail-closed policy limits the patch to `Cargo.toml`, `README.md`, and files under `crates/`, `tests/`, and `docs/`; limits the change to eight files and 50 KB; and rejects deletion, rename, symlink, binary, secret, workflow, lockfile, and deployment changes.
4. The runner applies the patch and executes formatting, Clippy with warnings denied, the full workspace test suite, and a release build.
5. A fresh reviewer pass inspects the task, plan, and diff. A rejected patch is not committed.
6. Only after local checks and review pass does the agent push to `agents/rust-autonomy`, create or update a draft PR, and dispatch **Rust CI**.
7. The next iteration starts only after that exact branch commit receives a successful Rust CI result. A failed CI run starts a bounded repair attempt using the failed job logs.

## Stop conditions

The cycle stops safely if:
- the model/runtime is unavailable or returns an invalid patch;
- checks or review cannot be made green within three patch attempts;
- the same CI task fails twice during repair;
- the planner cannot produce a valid next task;
- 40 agent iterations have been attempted.

A stop is not a success state. It requires human review before continuing.

## Model and independence

The current configured model is `qwen2.5-coder:7b`, served locally by Ollama on the runner. Planner, implementer, and reviewer are separate prompts and calls, but use the same model weights; the reviewer is role-separated, not an independent model/vendor. The workflow does not call Claude, Codex, or another hosted coding service, and does not require a separate model API key.

## Boundaries

- All work is confined to the new Rust repository and the `agents/rust-autonomy` branch.
- The old React application, production deployment, production database and production credentials are not accessed or changed.
- Model output cannot modify agent scripts, GitHub workflows, secrets, lockfiles, or deployment settings.
- No automatic merge or production deployment is enabled.
- Hidden admin/operations screens are deferred until primary user-facing flows have been migrated.
- Green CI proves only the checks that CI actually runs; it does not prove end-to-end feature parity or production readiness.
- Economic Dynasty is a planned domain, not a licence to interrupt core football parity, create unverified financial claims, or add live providers without a checked source/access/cost contract.

The workflow and state are visible in this directory. The first activation commit is marked with the `[agent-bootstrap]` prefix.
