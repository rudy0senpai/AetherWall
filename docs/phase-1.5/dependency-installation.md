# Phase 1.5 — Dependency Installation (CachyOS)

**Date:** 2026-11-13
**Branch:** `phase-1/core-implementation`
**Target OS:** CachyOS Linux (KDE Plasma 6)
**Development host OS:** Windows 11 (MSYS_NT-10.0-26300 — **NOT CachyOS**)

## 1. Operating system confirmation

| Property | Value |
|---|---|
| Host OS | Windows 11 (MSYS_NT-10.0-26300 via MSYS2) |
| Target OS | CachyOS Linux (KDE Plasma 6, Wayland) |
| Current session | Not a KDE Plasma session (no XDG_SESSION_TYPE) |
| Validation host | CachyOS required for KDE integration |

**Limitation:** This development environment runs Windows 11. KDE Plasma 6 and
all required Linux desktop dependencies cannot be installed or validated here.
The Phase 1.5 validation is therefore performed on a CachyOS machine.

## 2. Required dependencies

The following dependencies are required by the existing Phase 1 source code.

### 2.1 Build toolchain

| Package | Purpose | Required by |
|---|---|---|
| `rustup` / `cargo` | Rust toolchain | All Rust crates |
| `cmake` | Qt build integration | `apps/aetherwall-studio/CMakeLists.txt` |
| `ninja` | Fast build (optional, preferred with CMake) | Qt build |
| `gcc`/`g++` (GCC 12+) | C/C++ compilation | C runtime link, any native helpers |

### 2.2 Qt 6

| Package | Purpose |
|---|---|
| `qt6-base-dev` | Qt 6 Core, Gui, Widgets, Network |
| `qt6-declarative-dev` | QML/QtQml |
| `qt6-multimedia-dev` | QtMultimedia (for image runtime) |
| `qt6-svg-dev` | SVG support |
| `qt6-webengine-dev` | (optional) Web wallpaper engine |
| `qt6-wayland-dev` | Qt 6 Wayland integration |

### 2.3 D-Bus

| Package | Purpose |
|---|---|
| `dbus` | D-Bus system/bus daemon |
| `dbus-devel` / `dbus-glib` | D-Bus development headers |
| `d-feet` or `qdbus` | IPC testing (optional) |

### 2.4 KDE Plasma 6 (target desktop)

| Package | Purpose |
|---|---|
| `plasma` | KDE Plasma 6 desktop |
| `plasma-wayland-shell` | Wayland session |
| `kde-runtime` or `kde-baseapps` | Required Plasma libraries |
| `kde-kcm` / `kconfig` | KConfig for settings |

Actually, more precisely (CachyOS/AUR-based):

| Package | Purpose |
|---|---|
| `plasma` | KDE Plasma 6 |
| `kdedevel` | KDE development headers (KCoreAddons, KF5, etc.) |
| `kf5-kconfig` / `kf6-kconfig` | KConfig configuration |
| `kwindowsystem` / `kwayland` | Window system integration |
| `kwayland-backend` | Wayland backend for Plasma |
| `kwayland-server` | Wayland server (XDG-shell, etc.) |

### 2.5 Rust crates (via `cargo`)

All Rust dependencies are declared in `Cargo.toml` and are fetched
automatically by `cargo build`. System packages may be needed for:

- SQLite: `mesa` + `sqlite` (via `rustup` bundled or system `libsqlite3-dev`)
- D-Bus: `dbus` (via cargo `dbus` crate)

## 3. Installation commands

### 3.1 CachyOS / Arch installation

```bash
# 1. Install build toolchain
sudo pacman -Syu
sudo pacman -S rustup cmake ninja gcc make

# 2. Install Qt 6
sudo pacman -S qt6-base qt6-declarative qt6-multimedia qt6-svg

# 3. Install D-Bus development
sudo pacman -S dbus dbus-glib

# 4. Start D-Bus
systemctl --user enable dbus --now
dbus-daemon --session --fork

# 5. Install KDE Plasma 6 (complete desktop)
sudo pacman -S plasma
sudo systemctl set-default graphical.target

# 6. Log out and log in to KDE Plasma 6
```

### 3.2 Alternative: Conda / Miniforge (for testing ecosystem)

If the CachyOS host has limited developer tooling, a Conda environment can
be used to validate the Rust code and SQLite logic. However, **KDE integration
requires KDE Plasma 6** and cannot be validated outside of a KDE session.

## 4. Verification after installation

Run these checks on the CachyOS host:

```bash
# Rust toolchain
rustc --version
cargo --version

# Qt 6
pkg-config --modversion Qt6Core
pkg-config --modversion Qt6Qml
pkg-config --modversion Qt6Multimedia
pkg-config --modversion Qt6Wayland

# CMake and Ninja
cmake --version
ninja --version

# D-Bus
dbus-daemon --version
qdbus --version

# SQLite
sqlite3 --version

# KDE Plasma 6
plasmaserver --version
kbuildsycoca6 --version

# GPU and drivers
glxinfo | grep "OpenGL es profile"  # or use vulkaninfo
```

## 5. Validation matrix

| Check | Status | Evidence |
|---|---|---|
| Current OS is Windows 11 | ✅ Verified | MSYS_NT-10.0-26300 |
| Target OS is CachyOS | ✅ Documented | Target environment spec |
| GTK/QT modules verify | ❌ Blocked | No CachyOS host available |
| D-Bus service validation | ❌ Blocked | No CachyOS host available |
| Database validation | ✅ Partial | SQLite CLI available on Windows |
| QML syntax review | ✅ Partial | Manual review (no QML compiler) |
| Rust code review | ✅ Partial | Manual review (no rustc) |

## 6. Recommended CachyOS validation session

A CachyOS VM or physical machine with:
- KDE Plasma 6 (Wayland preferred)
- GPU drivers installed (NVIDIA or AMD)
- Rust toolchain and Qt 6 dev packages
- D-Bus running

Then, from that machine:
1. Install dependencies
2. Run `cargo build --workspace`
3. Run `cmake` and build the Studio
4. Run the daemon
5. Test D-Bus IPC
6. Test SQLite operations
7. Launch the Studio
8. Validate all 8 pages
9. Test image runtime
10. Test slideshow
11. Validate KDE Plasma 6 integration
12. Document results in `validation-report.md`

## 7. Known limitations

1. **No CachyOS host available.** The environment is Windows 11 (MSYS_NT-10.0-26300).
2. **KDE Plasma 6 integration blocked.** A KDE Plasma 6 session is required for validation.
3. **Qt 6 multimedia not available.** The image runtime cannot be compiled or tested on Windows.
4. **D-Bus IPC cannot be tested.** Requires a D-Bus session on the target OS.
5. **Image runtime cannot be tested.** Requires Qt 6 multimedia on Linux.
6. **Slideshow runtime cannot be tested.** Requires Qt 6 multimedia.

## 8. PostgreSQL / other databases — not used

AetherWall uses **SQLite** for all local persistence. No PostgreSQL, MySQL, or
other database systems are used.
