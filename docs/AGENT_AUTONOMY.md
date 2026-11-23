# Rust agent cycle

## Purpose

A repository-owned coding loop for the Rust rewrite. It uses a local Ollama model on a GitHub-hosted runner and separates planning, implementation, and review into role-specific model calls.

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

The workflow and state are visible in this directory. The first activation commit is marked with the `[agent-bootstrap]` prefix.
