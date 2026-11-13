# AetherWall Phase 1.5 — Dual-Environment Validation Plan

## Validation Matrix

| Test | Windows (MSYS2) | WSL2 Arch Linux | Status |
|---|---|---|---|
| Rust toolchain | ❌ Not installed | ✅ Rust 1.99.0 | PASS (WSL) |
| CMake | ❌ Not installed | ✅ CMake 4.4.5 | PASS (WSL) |
| Ninja | ❌ Not installed | ✅ Ninja 1.13.2 | PASS (WSL) |
| GCC/G++ | ✅ GCC 16.2.0 | ✅ GCC 16.2.0 (MSYS2) | PARTIAL |
| D-Bus | ❌ Not installed | ✅ D-Bus 1.16.2 | PARTIAL (WSL) |
| Qt 6 | ❌ Not installed | ⚠️ qmake6 only, no headers | BLOCKED |
| SQLite | ✅ SQLite 3.53.4 CLI | ✅ SQLite 3.53.4 (WSL) | PARTIAL |
| Rust workspace build | ❌ No toolchain | ❌ Member resolution issue | BLOCKED |
| D-Bus validation | ❌ No tools | ❌ No session bus | BLOCKED |
| Qt/QML Studio | ❌ No Qt | ❌ No dev headers | BLOCKED |
| Image runtime | ❌ No Qt multimedia | ❌ No Qt dev | BLOCKED |

## Exact Test Commands

### WSL Arch Linux
```bash
# Rust toolchain
rustc --version
cargo --version

# Workspace build
cd /home/dhruv/aetherwall-clean
cargo check --workspace

# D-Bus test
dbus-daemon --version

# SQLite
sqlite3 --version

# Qt 6 check
qmake6 --version
```

### Windows MSYS2
```bash
# GCC
gcc --version
g++ --version

# pkg-config
pkg-config --version

# SQLite
sqlite3 --version

# Git
git --version
```

## Known Issues

1. **WSL workspace member resolution**: Cargo 1.99.0 on WSL does not resolve workspace members from a directory with spaces in the path.
2. **Qt 6 dev headers**: qmake6 binary exists but Qt 6 development headers are not installed.
3. **Rust on Windows**: No Rust toolchain is available on the MSYS2 Windows environment.
4. **CMake/Ninja on Windows**: Missing on Windows, only available on WSL.
5. **D-Bus**: No D-Bus tools on Windows, D-Bus daemon exists on WSL but no session bus configured.

## Repository Audit Results (2026-11-13)

- **Branch:** `phase-1/core-implementation`
- **Commit:** `f1701a1` and `8f499f8`
- **Workspace:** 10 crates (9 in workspace members + `aetherwall-display`)
- **Crate files:** 10 `Cargo.toml` files detected in `crates/`
- **Workspace members:** 10 (fixed)
- **Issues found:** 2 crates missing `Cargo.toml` → FIXED
- **Build status:** Blocked by WSL cargo member resolution
