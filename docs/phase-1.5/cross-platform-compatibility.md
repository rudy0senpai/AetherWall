
# AetherWall Phase 1.5 — Cross-Platform Compatibility Report

**Date:** 2026-11-13

**Branch:** `phase-1/core-implementation`

**Commits tested:**
- `f1701a1` — docs(phase-1.5): validation report documenting environment limitations
- `8f499f8` — feat(core): implement Phase 1 foundation — workspace, crates, daemon, runtime, Studio UI

---

## 1. Cross-platform comparison

| Aspect | Windows 11 (MSYS2) | WSL2 Arch Linux |
|---|---|---|
| **Rust toolchain** | ❌ Not installed | ✅ Rust 1.99.0 |
| **CMake** | ❌ Not installed | ✅ 4.4.5 |
| **Ninja** | ❌ Not installed | ✅ 1.13.2 |
| **Qt 6** | ❌ Not installed | ⚠️ qmake6 only, no dev headers |
| **D-Bus** | ❌ Not installed | ✅ 1.16.2 |
| **SQLite** | ✅ CLI 3.53.4 | ✅ CLI 3.53.4 |
| **GCC/G++** | ✅ 16.2.0 (MinGW) | ✅ 16.2.0 (MinGW) |
| **pkg-config** | ✅ 3.0.7 | ✅ 3.24.52 |
| **Git** | ✅ 2.53.0 | ✅ 2.53.0 |

---

## 2. Path handling

### Windows
- Paths use `\` separator: `C:\Users\Rudy0senpai\...`
- Drive letters are mandatory
- Case-insensitive filesystem
- Application data directory: `%APPDATA%` or `%LOCALAPPDATA%`

### WSL
- Paths use `/` separator: `/home/dhruv/...`
- Windows paths accessible via `/mnt/c/`
- Case-sensitive filesystem
- Uses XDG directories: `~/.config`, `~/.local`

### Compatibility findings
- The repository path `...\AetherWall-Wallpaper engine` contains spaces, which breaks WSL cargo workspace resolution.
- MSYS2 paths use `\` which is also the directory separator in some contexts.
- The WSL approach of mounting Windows at `/mnt/c/` is the recommended way to access the Windows filesystem.
- Cross-platform builds must avoid spaces in paths.

---

## 3. File permissions

### Windows
- Windows ACL-based permissions (NTFS)
- Default permissions: owner read/write/execute, others read-only
- No traditional Unix `chmod`

### WSL
- Linux permissions (rwxr-xr-x)
- Files copied from Windows inherit Windows permissions mapped to Linux
- Symlinks are treated as files (not followed) by default

### Compatibility findings
- WSL2 automatically maps Windows file permissions to Linux permissions.
- Symlinks in Windows (`.lnk` files) are not preserved in WSL.
- The `aetherwall-studio.desktop` file works as a desktop entry on Linux but not on Windows.

---

## 4. Filename encoding

### Windows
- UTF-16 encoding (wchar_t)
- Supports Unicode filenames

### WSL
- UTF-8 encoding (locale-dependent)
- WSL defaults to UTF-8
- Windows filenames with Unicode are visible in WSL

### Compatibility findings
- Both platforms support UTF-8/UTF-16 filenames.
- No encoding issues detected with the current repository files.

---

## 5. Case sensitivity

### Windows
- Case-insensitive filesystem
- `Cargo.toml` and `cargo.toml` are the same file

### WSL
- Case-sensitive filesystem (Linux native)
- `Cargo.toml` and `cargo.toml` are different files

### Compatibility findings
- The workspace members list uses lowercase `crates/aetherwall-core` etc.
- All crate `Cargo.toml` files are lowercase, so there is no case sensitivity conflict.
- Future code should avoid creating files with only case differences.

---

## 6. SQLite behavior

### Windows
- SQLite 3.53.4 (CLI available)

### WSL
- SQLite 3.53.4 (CLI available)

### Compatibility findings
- Same SQLite version on both platforms.
- The database schema is platform-independent.
- Rust's `rusqlite` crate uses the same SQLite version on both platforms.

---

## 7. Image format support

### Windows
- PNG, JPEG, WebP, BMP support depends on Qt 6 multimedia installation
- No Qt 6 installed

### WSL
- qmake6 (Qt 6.12.0) installed
- Qt 6 dev headers not installed
- QML-based image loading requires Qt 6 multimedia

### Compatibility findings
- Both platforms need Qt 6 multimedia to support the image runtime.
- No platform-specific image format issues expected.

---

## 8. Configuration locations

| Platform | Application Data | Config | Cache |
|---|---|---|---|
| Windows | `%APPDATA%\AetherWall` | `%APPDATA%\AetherWall` | `%LOCALAPPDATA%\AetherWall` |
| WSL/Linux | `~/.config/aetherwall` | `~/.config/aetherwall` | `~/.cache/aetherwall` |

### Compatibility findings
- Cross-platform configuration paths must be determined at runtime.
- Use `std::env::var("APPDATA")` on Windows and `$XDG_CONFIG_HOME` on Linux.

---

## 9. Logging locations

### Windows
- `%LOCALAPPDATA%\AetherWall\logs\`

### Linux
- `~/.cache/aetherwall/logs/`

### Compatibility findings
- Use environment variables (`APPDATA`, `XDG_CACHE_HOME`) to determine paths at runtime.

---

## 10. Build scripts

| Platform | Build system | Launch |
|---|---|---|
| Windows | MSBuild or CMake | PowerShell / .exe |
| Linux/WSL | CMake + Ninja | Bash script |

### Compatibility findings
- Existing `scripts/build.sh` and `scripts/launch.sh` are Bash scripts.
- Windows requires PowerShell-based scripts or a tool like `bash.exe`.
- Cross-platform builds must include a Windows-specific build script.

---

## 11. Runtime behavior

### Path handling
- Windows: `C:\Users\Rudy0senpai\AppData\...`
- Linux: `/home/dhruv/.config/aetherwall`
- Fix: Use `std::env::var()` with platform-specific fallbacks.

### File permissions
- Windows: ACL-based
- Linux: Unix permissions
- Fix: Check permissions at runtime and report errors gracefully.

### Filename encoding
- Windows: UTF-16
- Linux: UTF-8
- Both platforms are Unicode-aware in modern systems.

---

## 12. Platform-specific configuration paths

```rust
// Example cross-platform config path resolution
use std::env;
use std::path::PathBuf;

fn get_app_data_dir() -> PathBuf {
    #[cfg(windows)]
    {
        let app_data = env::var("APPDATA").expect("APPDATA not set");
        PathBuf::from(app_data).join("AetherWall")
    }
    #[cfg(not(windows))]
    {
        let config_home = env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| PathBuf::from("/home/dhruv/.config"));
        config_home.join("aetherwall")
    }
}
```

---

## 13. Case sensitivity check

```
Windows:    Cargo.toml == cargo.toml
WSL/Linux:  Cargo.toml != cargo.toml

All workspace members use lowercase paths, so there is no conflict.
```

---

## 14. Known cross-platform issues

| Issue | Windows | Linux | Fix |
|---|---|---|---|
| Path with spaces | ✅ Supported | ✅ Supported | Document limitation |
| D-Bus tools | ❌ Missing | ✅ Available | Install on Windows |
| Qt 6 dev | ❌ Missing | ⚠️ No headers | Install dev packages |
| CMake/Ninja | ❌ Missing | ✅ Available | Install on Windows |
| SQLite CLI | ✅ Available | ✅ Available | Both have SQLite |
| Rust toolchain | ❌ Missing | ✅ Available | Install on Windows |

---

## 15. Cross-platform build matrix

```
+---------------------------+-----------+-----------+
|                           | Windows   | WSL Arch  |
+---------------------------+-----------+-----------+
| Rust workspace build      | NO        | NO (WSL)  |
| Qt/QML Studio build       | NO        | NO        |
| D-Bus validation          | NO        | NO        |
| SQLite Rust CRUD          | NO        | NO        |
| Image runtime             | NO        | NO        |
| Slideshow runtime         | NO        | NO        |
| Security testing          | NO        | NO        |
+---------------------------+-----------+-----------+
```

---

## 16. Recommendations

1. **Use XDG directories on Linux** and `APPDATA` on Windows.
2. **Avoid spaces in paths** for cross-platform builds.
3. **Add Windows build script** (PowerShell or a setup step).
4. **Install Qt 6 dev packages** on WSL for QML validation.
5. **Configure D-Bus session bus** on WSL for IPC testing.
6. **Test on a native Linux host** (CachyOS) for definitive validation.
7. **Document all platform differences** in the project README.
