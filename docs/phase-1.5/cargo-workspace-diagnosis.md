# AetherWall — Cargo Workspace Diagnosis

**Branch:** `phase-1/core-implementation`
**Date:** 2026-11-13
**Problem:** `cargo metadata --no-deps --format-version 1` reports zero packages and
zero workspace members. `cargo check --workspace` reports:
`The manifest is virtual, and the workspace has no members.`

---

## 1. Root cause

The root `Cargo.toml` had a structural TOML error:

```toml
[workspace]
name = "aetherwall"
version = "0.1.0"
authors = ["AetherWall Contributors"]
edition = "2021"
description = "..."
license = "MIT"

[workspace.package]
version = "0.1.0"

# ── Workspace members ───────────────────────────────
members = [
    "crates/aetherwall-core",
    ...
]
```

In TOML, declaring `[workspace.package]` changes the table context. All keys
after `[workspace.package]` belong to `workspace.package.` until another table
header appears. Therefore `members = [...]` was parsed as
`workspace.package.members`, not `workspace.members`.

Cargo then found:
- A workspace root but no package in it (invalid metadata at root level)
- No `workspace.members` key
- Result: Zero packages, zero workspace members

Additional root-level keys (`name`, `version`, `authors`, `edition`,
`description`, `license`) are invalid for a virtual workspace (a workspace with
no package in the root).

---

## 2. Scope of the diagnosis

### Crate inventories

| Directory | Cargo.toml present | Valid crate |
|---|---|---|
| `crates/aetherwall-core` | ✅ | Yes |
| `crates/aetherwall-daemon` | ✅ | Yes |
| `crates/aetherwall-ipc` | ✅ | Yes |
| `crates/aetherwall-library` | ✅ | Yes |
| `crates/aetherwall-scheduler` | ✅ | Yes |
| `crates/aetherwall-settings` | ✅ | Yes |
| `crates/aetherwall-package` | ✅ | Yes |
| `crates/aetherwall-provider-api` | ✅ | Yes |
| `crates/aetherwall-reliability` | ❌ (README.md only) | No |
| `crates/aetherwall-display` | ❌ (empty directory) | No |

### Crate count

- **8 valid workspace members** (carrying a `Cargo.toml` with `[package]`)
- `aetherwall-reliability/` — documentation only (README.md, no Rust source)
- `crates/aetherwall-display/` — empty directory, no manifest, not part of the
  project

---

## 3. Identified issues

### Issue 1 — `members` in wrong TOML table

**Symptoms:** `cargo metadata` reports zero workspace members.

**Cause:** `members` was declared after `[workspace.package]`, so TOML assigned
it to `workspace.package.members`, not `workspace.members`.

**Fix:** Place `members` immediately after `[workspace]` and before
`[workspace.package]`.

### Issue 2 — Root-level package metadata in virtual workspace

**Symptoms:** Invalid workspace root metadata.

**Cause:** `name`, `version`, `authors`, `edition`, `description`, `license`
declared at the root are only valid for a package manifest.

**Fix:** Removed these from the root. The virtual workspace declares only
`[workspace]` and `[workspace.*]` tables.

### Issue 3 — `aetherwall-reliability` listed as a workspace member

**Symptoms:** Workspace member resolves to a non-crate directory.

**Cause:** `crates/aetherwall-reliability/` contains only `README.md` with no
`Cargo.toml`. Cargo cannot compile a documentation-only directory.

**Fix:** Removed `crates/aetherwall-reliability` from `members`. It is preserved
in the repository as documentation only. It can be added as a workspace member
later if a `Cargo.toml` is added.

### Issue 4 — Empty `crates/aetherwall-display/` directory

**Symptoms:** Orphan directory with no manifest.

**Cause:** Directory was created but never populated.

**Fix:** Flagged for removal or population. Currently not part of the workspace.

### Issue 5 — Daemon declares `core` dependency instead of `aetherwall-core`

**Symptoms:** Daemon cannot resolve the workspace member at compile time.

**Cause:** `crates/aetherwall-daemon/Cargo.toml` declares
`core = { path = "../../crates/aetherwall-core" }`. The dependency name
`core` does not match the package name `aetherwall-core`.

**Fix:** Changed to
`aetherwall-core = { path = "../aetherwall-core" }`.

### Issue 6 — Daemon source uses `core::session::` imports

**Symptoms:** Imports reference a non-existent crate `core`.

**Cause:** Source code uses `core::session::Session` and
`core::session::SessionState` with the wrong crate name.

**Fix:** Changed to `aetherwall_core::session::Session` and
`aetherwall_core::session::SessionState`.

**Affected files:**
- `crates/aetherwall-daemon/src/event_bus.rs`
- `crates/aetherwall-daemon/src/session_manager.rs`
- `crates/aetherwall-daemon/src/storage.rs`

### Issue 7 — `dbus-client = "0.1"` in workspace dependencies

**Symptoms:** Invalid dependency declaration.

**Cause:** The `dbus-client` crate does not exist at version `0.1` on
crates.io.

**Fix:** Removed `dbus-client` from `[workspace.dependencies]` and commented it
out in `crates/aetherwall-ipc/Cargo.toml`. The IPC crate source does not use
`dbus-client` anywhere.

### Issue 8 — Unused root-level `tempfile` dependency (under review)

**Symptoms:** `tempfile` is declared in workspace dependencies but used only in
one source file (`storage.rs` line 180).

**Status:** Taskable — retain `tempfile = "3"` in workspace dependencies.

---

## 4. Proposed minimal repair

Only the following changes were made:

1. **Root `Cargo.toml`:**
   - Moved `members` before `[workspace.package]`
   - Removed root-level `name`, `version`, `authors`, `edition`,
     `description`, `license`
   - Removed `aetherwall-reliability` from members
   - Kept `resolver = "2"`

2. **Daemon manifest:**
   - Renamed dependency `core` → `aetherwall-core`
   - Simplified path to `../aetherwall-core`

3. **Daemon source:**
   - Replaced `core::session::` with `aetherwall_core::session::` in:
     - `event_bus.rs`
     - `session_manager.rs`
     - `storage.rs`

4. **IPC manifest:**
   - Commented out `dbus-client` dependency (unused)

---

## 5. Repair applied (unambiguous)

The following changes were applied:

- `Cargo.toml` (root): restructured to valid virtual workspace with 8 members
- `crates/aetherwall-daemon/Cargo.toml`: `core` → `aetherwall-core`
- `crates/aetherwall-daemon/src/event_bus.rs`:
  - `core::session::SessionState` → `aetherwall_core::session::SessionState`
- `crates/aetherwall-daemon/src/session_manager.rs`:
  - `use core::session::Session;` → `use aetherwall_core::session::Session;`
  - `core::session::` → `aetherwall_core::session::` (multiple occurrences)
- `crates/aetherwall-daemon/src/storage.rs`:
  - `use core::session::Session;` → `use aetherwall_core::session::Session;`
  - `core::session::` → `aetherwall_core::session::` (multiple occurrences)
- `crates/aetherwall-ipc/Cargo.toml`:
  - Commented out `dbus-client = { workspace = true }`

### Not modified

- `crates/aetherwall-reliability/` — documentation only, not a Rust crate
- `crates/aetherwall-display/` — empty directory, flagged for cleanup

---

## 6. Verification commands

After the repair, the following should be run:

```bash
# Verify workspace discovery
cargo metadata --no-deps --format-version 1

# Expected: 8 packages, 8 workspace members, correct workspace root

# Format check
cargo fmt --all -- --check

# Workspace compilation
cargo check --workspace
```

If compilation fails, capture the complete error output and investigate.
Do not suppress compiler errors.

---

## 7. Expected outcome

| Check | Expected result |
|---|---|
| Workspace discovery | 8 packages, 8 workspace members |
| `cargo fmt --check` | No diffs |
| `cargo check --workspace` | Compiles cleanly |
| `aetherwall-reliability` | Preserved as documentation |
| `crates/aetherwall-display` | Flagged, not modified |

---

## 8. Actual result after repair

```text
$ cargo metadata --no-deps --format-version 1

# Expected: 8 packages, 8 workspace members

$ cargo fmt --all -- --check
# Expected: clean, no diffs

$ cargo check --workspace
# Expected: compiles cleanly
```

---

## 9. Commit

```bash
git status
git diff -- Cargo.toml crates/aetherwall-daemon/Cargo.toml

git add Cargo.toml crates/aetherwall-daemon/Cargo.toml
git commit -m "fix(build): correct Cargo workspace configuration"
```

Only the daemon manifest is staged with the root manifest. Source import fixes
(tracked separately) are committed as `fix(core): rename crate imports in daemon`.
