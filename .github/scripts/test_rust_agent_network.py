import importlib.util
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location(
    "rust_agent_network",
    Path(__file__).with_name("rust_agent_network.py"),
)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def changed_file(path="crates/api/src/lib.rs", body="+pub fn ready() -> bool { true }"):
    return (
        f"diff --git a/{path} b/{path}\n"
        f"--- a/{path}\n"
        f"+++ b/{path}\n"
        "@@ -1 +1 @@\n"
        "-pub fn ready() -> bool { false }\n"
        f"{body}\n"
    )


def new_file(path="crates/web/src/lib.rs"):
    return (
        f"diff --git a/{path} b/{path}\n"
        "new file mode 100644\n"
        "--- /dev/null\n"
        f"+++ b/{path}\n"
        "@@ -0,0 +1 @@\n"
        "+pub fn web_ready() -> bool { true }\n"
    )


class AgentTriggerPolicyTests(unittest.TestCase):
    def test_successful_main_push_bootstraps_agents(self):
        self.assertTrue(MODULE.qualifies_for_agent({
            "head_branch": "main",
            "event": "push",
            "conclusion": "success",
        }))

    def test_main_pull_request_success_does_not_bootstrap_agents(self):
        self.assertFalse(MODULE.qualifies_for_agent({
            "head_branch": "main",
            "event": "pull_request",
            "conclusion": "success",
        }))

    def test_failed_main_push_does_not_bootstrap_agents(self):
        self.assertFalse(MODULE.qualifies_for_agent({
            "head_branch": "main",
            "event": "push",
            "conclusion": "failure",
        }))

    def test_agent_branch_dispatch_can_advance_or_repair(self):
        for conclusion in ("success", "failure"):
            with self.subTest(conclusion=conclusion):
                self.assertTrue(MODULE.qualifies_for_agent({
                    "head_branch": "agents/rust-autonomy",
                    "event": "workflow_dispatch",
                    "conclusion": conclusion,
                }))

    def test_unrelated_branches_never_start_agents(self):
        self.assertFalse(MODULE.qualifies_for_agent({
            "head_branch": "feature/other",
            "event": "push",
            "conclusion": "success",
        }))


class AgentWritePolicyTests(unittest.TestCase):
    def test_rust_and_docs_paths_are_writable(self):
        for path in (
            "Cargo.toml",
            "README.md",
            "crates/api/src/main.rs",
            "crates/web/src/lib.rs",
            "tests/api_contract.rs",
            "docs/MIGRATION_SCOPE.md",
        ):
            with self.subTest(path=path):
                self.assertTrue(MODULE.allowed_path(path))

    def test_workflow_and_state_paths_are_not_model_writable(self):
        for path in (
            ".github/workflows/ci.yml",
            ".github/scripts/rust_agent_network.py",
            ".github/agent-state.json",
            "supabase/migrations/001.sql",
            "src/App.tsx",
        ):
            with self.subTest(path=path):
                self.assertFalse(MODULE.allowed_path(path))

    def test_rejects_traversal_secrets_and_lockfiles(self):
        for path in (
            "../Cargo.toml",
            "crates/../../.env",
            "crates/.env.production",
            "crates/private/token.rs",
            "Cargo.lock",
            "crates\\api\\main.rs",
            "/tmp/payload.rs",
        ):
            with self.subTest(path=path):
                self.assertFalse(MODULE.allowed_path(path))

    def test_accepts_modified_file_patch(self):
        self.assertEqual(
            MODULE.validate_patch(changed_file()),
            ["crates/api/src/lib.rs"],
        )

    def test_accepts_regular_new_file_patch(self):
        self.assertEqual(
            MODULE.validate_patch(new_file()),
            ["crates/web/src/lib.rs"],
        )

    def test_rejects_unapproved_path(self):
        with self.assertRaises(ValueError):
            MODULE.validate_patch(changed_file(".github/workflows/ci.yml"))

    def test_rejects_file_deletion(self):
        patch = (
            "diff --git a/crates/api/src/old.rs b/crates/api/src/old.rs\n"
            "deleted file mode 100644\n"
            "--- a/crates/api/src/old.rs\n"
            "+++ /dev/null\n"
        )
        with self.assertRaises(ValueError):
            MODULE.validate_patch(patch)

    def test_rejects_rename(self):
        patch = (
            "diff --git a/crates/api/src/a.rs b/crates/api/src/b.rs\n"
            "similarity index 100%\n"
            "rename from crates/api/src/a.rs\n"
            "rename to crates/api/src/b.rs\n"
        )
        with self.assertRaises(ValueError):
            MODULE.validate_patch(patch)

    def test_rejects_symlink(self):
        patch = new_file("crates/web/link").replace(
            "new file mode 100644", "new file mode 120000"
        )
        with self.assertRaises(ValueError):
            MODULE.validate_patch(patch)

    def test_rejects_more_than_eight_files(self):
        patch = "".join(changed_file(f"crates/api/file{i}.rs") for i in range(9))
        with self.assertRaises(ValueError):
            MODULE.validate_patch(patch)

    def test_rejects_oversized_patch(self):
        with self.assertRaises(ValueError):
            MODULE.validate_patch(changed_file(body="+" + "x" * MODULE.MAX_PATCH_BYTES))

    def test_patch_must_match_tracked_file_state(self):
        self.assertEqual(
            MODULE.validate_patch(
                changed_file(),
                existing_paths={"crates/api/src/lib.rs"},
            ),
            ["crates/api/src/lib.rs"],
        )
        with self.assertRaisesRegex(ValueError, "not tracked"):
            MODULE.validate_patch(
                changed_file("crates/api/src/missing.rs"),
                existing_paths={"crates/api/src/lib.rs"},
            )

    def test_new_file_patch_cannot_overwrite_existing_path(self):
        with self.assertRaisesRegex(ValueError, "new file"):
            MODULE.validate_patch(
                new_file("crates/web/src/lib.rs"),
                existing_paths={"crates/web/src/lib.rs"},
            )

    def test_web_client_rejects_server_runtime_code(self):
        patch = changed_file(
            "crates/web/src/main.rs",
            body="+    axum::serve(listener, app).await.unwrap();",
        )
        with self.assertRaisesRegex(ValueError, "WASM client architecture"):
            MODULE.validate_architecture(patch, ["crates/web/src/main.rs"])

    def test_root_workspace_rejects_package_conversion(self):
        patch = changed_file("Cargo.toml", body="+[package]")
        with self.assertRaisesRegex(ValueError, "workspace manifest"):
            MODULE.validate_architecture(patch, ["Cargo.toml"])

    def test_web_manifest_rejects_server_only_dependencies(self):
        patch = changed_file(
            "crates/web/Cargo.toml",
            body="+axum = { version = '0.8' }",
        )
        with self.assertRaisesRegex(ValueError, "server-only"):
            MODULE.validate_architecture(patch, ["crates/web/Cargo.toml"])

    def test_local_model_runtime_has_bounded_cpu_friendly_limits(self):
        self.assertEqual(MODULE.MODEL, "qwen2.5-coder:3b")
        self.assertEqual(MODULE.MODEL_CONTEXT_TOKENS, 6144)
        self.assertEqual(MODULE.MODEL_REQUEST_TIMEOUT_SECONDS, 300)

    def test_architecture_contract_is_explicit(self):
        self.assertIn("Leptos 0.8", MODULE.ARCHITECTURE_CONTRACT)
        self.assertIn("Do not put Axum routers", MODULE.ARCHITECTURE_CONTRACT)
        self.assertIn("workspace manifest", MODULE.ARCHITECTURE_CONTRACT)


if __name__ == "__main__":
    unittest.main()
