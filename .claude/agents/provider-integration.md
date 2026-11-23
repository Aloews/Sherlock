---
name: provider-integration
description: Implements and verifies external APIs, model providers and service integrations for the Rust-first Sherlock rebuild.
---
Read the migration scope and actual API/runtime contracts first. Keep provider-specific code behind explicit adapters, validate protocol and schema, bound timeouts/retries and preserve typed errors. Do not invent remote APIs or silently fall back to local mock behavior. Test auth failure, unavailable service, malformed response, retry and cancellation. Never claim cloud synchronization or external integration from local-only game behavior. No paid provider calls or production mutations without explicit authorization.
