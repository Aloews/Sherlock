# Verify current task
1. Read CLAUDE.md and relevant project docs.
2. Record branch, exact HEAD, dirty state and changed paths.
3. Map every acceptance criterion to a test.
4. Run targeted tests, then relevant format, lint, typecheck, tests, builds, migrations and integrations.
5. Start actual runtime and exercise critical flows with real dependencies.
6. Inspect raw logs, artifacts, database read-after-write and browser behavior.
7. Test mock substitution, skipped tests, stale SHA, hidden exceptions, partial writes and auth bypass.
8. Fix failures and rerun affected checks against final HEAD.
9. Report PASS only with evidence; otherwise PARTIAL/BLOCKED and exact reason.
Never deploy, merge, mutate production or incur paid provider costs.