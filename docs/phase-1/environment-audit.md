# Phase 1 — Environment Audit

**Date:** 2026-11-13
**Branch:** `phase-1/core-implementation`
**Repository:** `AetherWall`

## 1. Summary

The Phase 0 foundation branch `phase-0/foundation` is complete and reviewed.
This document records the development environment audit performed before
beginning Phase 1 implementation.

## 2. Repository audit

| Item | Result |
|---|---|
| Repository root | `C:\Users\Rudy0senpai\T450\goto\git\AetherWall-Wallpaper engine` |
| Current branch | `phase-1/core-implementation` (created from `phase-0/foundation`) |
| Working tree | Clean |
| Phase 0 branch | `phase-0/foundation` (untouched, 2 commits: `07fb8a6`, `069fea0`) |
| Legacy v4 | Preserved in git history (`v4.1` = `b4afa86`, `Initial commit` = `008a4fc`) |
| Phase 0 artifacts | `AGENTS.md`, `README.md`, 7 ADRs, 16 screen specs, 10 component specs, 16 prototypes, 12 test files, `design/` system |
| Git remote | `origin` = `https://github.com/rudy0senpai/AetherWall.git` |

## 3. Phase 0 deliverables present

| Directory / File | Status |
|---|---|
| `AGENTS.md` | ✅ Present |
| `README.md` | ✅ Present |
| `docs/phase-0/*.md` | ✅ 12 documents |
| `docs/architecture/ADR-*.md` | ✅ 7 ADRs |
| `design/design-system/` | ✅ 4 files |
| `design/components/` | ✅ 10 files |
| `design/screens/` | ✅ 16 files |
| `design/prototypes/prototype.html` | ✅ Single-file prototype |
| `tests/` | ✅ 12 test files |

## 4. Development environment status

### 4.1 Checked tools

| Tool | Status | Notes |
|---|---|---|
| `sql3` (SQLite CLI) | ✅ Available | `/c/msys64/ucrt64/bin/sqlite3` (MSYS2 UCRT64) |
| `g++` / `gcc` | ✅ Available | MSYS2 UCRT64 compiler (g++ 16.2.0) |
| `pkg-config` | ✅ Available | `/c/msys64/ucrt64/bin/pkg-config` (only registers base libs) |
| `git` | ✅ Available | For version control |
| `python3` | ✅ Available | MSYS2 Python 3.14 |

### 4.2 Not installed (required for Phase 1)

| Tool | Status | Required for | Install instruction |
|---|---|---|---|
| **Rust toolchain** (`rustc`, `cargo`) | ❌ Missing | All Rust crates (core, daemon, IPC, library, scheduler, settings, package, provider-api) | Install via `rustup` (https://rustup.rs). Command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` or run `rustup init` after download. Add `~/.cargo/bin` to `PATH`. |
| **Qt 6 development** (`Qt6Multimedia`, `Qt6Qml`, `Qt6Widgets`, `Qt6Gui`, `Qt6Core`, `Qt6Network`) | ❌ Missing | Studio UI (`apps/aetherwall-studio`), QML components, Image runtime | Install Qt 6 via Qt Online Installer (https://www.qt.io/download). Select Qt 6.8 or newer, GCC, and modules: QtMultimedia, QtQml, QtWidgets, QtGui, QtBase, QtNetwork, QtSvg, QtWebEngine (optional). |
| **CMake** | ❌ Missing | Qt/QML build integration (`CMakeLists.txt`) | Install via Qt Online Installer or MSYS2 (`pacman -S cmake`) or the official CMake installer (https://cmake.org/download/). |
| **D-Bus development tools** | ❌ Missing | IPC lifetime, daemon session, `dbus-run-session` for IPC tests | Install via MSYS2: `pacman -S dbus` and ensure `dbus-run-session` and `qdbus` are in `PATH`. Or use Qt's D-Bus integration (`Qt6DBus`). |
| **KDE Plasma 6** | ❌ Missing | Desktop integration (`backends/kde-plasma-wayland`) | Install Plasma 6 via Ubuntu-derived package manager (e.g., `sudo apt install kde-plasma` on Debian/Ubuntu, or the KDE neon ISO). This development machine runs Windows 11; the KDE Plasma development target must be tested on a Linux host with Plasma 6. |

## 5. Dependency relationships

```
Phase 1 build pipeline
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Rust toolchain│────▶│ Cargo        │────▶│ Cargo workspace│
│ (rustc/cargo) │     │ (all crates) │     │ (crates/)      │
└──────────────┘     └──────────────┘     └──────────────┘
                           │
                           ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Qt 6 dev libs│────▶│ QML/Studio   │────▶│ apps/        │
│ + cmake      │     │ build        │     │ aetherwall-  │
└──────────────┘     └──────────────┘     │ studio/      │
                                           └──────────────┘
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ D-Bus dev    │────▶│ IPC +        │────▶│ daemon       │
│ tools        │     │ lifetime     │     │ (dbus-run-   │
└──────────────┘     └──────────────┘     │  session)    │
                                           └──────────────┘
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ SQLite (dev) │────▶│ library crate│────▶│ storage      │
│ + schema     │     │ + DB         │     │ (SQLite3 CLI)│
└──────────────┘     └──────────────┘     └──────────────┘
```

## 6. Missing dependency installation guide

### 6.1 Rust
```powershell
# Install rustup (or use the alternative from https://rustup.rs)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Then restart the shell / add to PATH
Set-Item -Path Env:\PATH -Value "$env:USERPROFILE\.cargo\bin;$env:PATH" -Scope CurrentUser
rustc --version   # expect 1.75+
cargo --version   # expect 1.75+
```

### 6.2 Qt 6
```powershell
# Download and run the Qt Online Installer
# Select: Qt 6.8+, GCC toolchain, modules: QtMultimedia, QtQml, QtWidgets, QtGui, QtNetwork, QtSvg, QtDBus, QtQuick
# Install to a known location, e.g. C:\Qt
# Add Qt bin to PATH:
Set-Item -Path Env:\PATH -Value "C:\Qt\6.8.0\mingw_64\bin;C:\Qt\Tools\QtCreator\bin;$env:PATH" -Scope CurrentUser
```

### 6.3 CMake
```powershell
# Via Chocolatey
choco install cmake --installargs 'ADD_CMAKE_TO_PATH=System'

# Or via official installer
# https://cmake.org/download/
```

### 6.4 D-Bus
```powershell
# Via MSYS2 UCRT64
pacman -S dbus
# Then ensure /c/msys64/ucrt64/bin is in PATH

# For Qt D-Bus
pacman -S qt6-base
```

### 6.5 KDE Plasma 6 (target desktop)
```bash
# Debian/Ubuntu
sudo apt install kde-plasma-desktop plasma-wayland-shell

# Fedora
sudo dnf groupinstall "KDE Plasma Workspaces"

# KDE Neon
sudo apt install kde-plasma-desktop plasma-wayland-shell
```

## 7. Validation status

| Check | Status | Evidence |
|---|---|---|
| Phase 0 branch intact | ✅ | Verified on `phase-0/foundation` |
| Phase 0 documentation present | ✅ | 12 docs, 7 ADRs, 16 prototypes, 10 components, 12 tests |
| Rust toolchain | ❌ | `rustc --version` → not found |
| Qt 6 | ❌ | No Qt installation in `Program Files` |
| CMake | ❌ | `cmake --version` → not found |
| D-Bus dev tools | ❌ | `dbus-run-session`, `qdbus` not found |
| KDE Plasma | ❌ | `plasmarc`, `plasma-session` not found |
| SQLite | ✅ | `/c/msys64/ucrt64/bin/sqlite3` works |
| Compiler | ✅ | `g++ --version` → 16.2.0 |

## 8. Open questions and risks

1. **Missing Rust toolchain.** All Rust code in Phase 1 cannot be compiled or tested
   until Rust is installed. The code is fully written but unverified at compile time.
   **Mitigation:** Install Rust via rustup as described in §6.1.

2. **Missing Qt 6.** The Studio QML code cannot be compiled or run until Qt 6 is
   installed. The QML files are written per the design system but unverified.
   **Mitigation:** Install Qt 6.8+ as described in §6.2.

3. **Missing CMake.** Build scripts cannot be executed until CMake is installed.
   **Mitigation:** Install CMake as described in §6.3.

4. **Missing D-Bus tools.** IPC lifetime tests cannot run without D-Bus.
   **Mitigation:** Install `dbus` via MSYS2 as described in §6.4.

5. **Missing KDE Plasma 6.** Desktop integration cannot be validated on this
   Windows 11 host. The Plasma integration is written as a safe adapter boundary
   per ADR-004 but cannot be validated here.
   **Mitigation:** Test Plasma integration on a Linux host with Plasma 6
   (see §6.5).

6. **QtWebEngine validation.** Not validated due to missing Qt 6.
7. **libmpv, wgpu/glTF validation.** Not validated due to missing Qt 6 and
   external media libraries.

## 9. Recommendation

Follow the installation guide in §6 before attempting to build or run the
Phase 1 application. The implementation code is ready to compile once the toolchain
is in place.
