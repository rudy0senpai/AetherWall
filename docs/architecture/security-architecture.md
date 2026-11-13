# AetherWall Security Architecture — Phase 0

## 1. Scope

This document defines the security boundaries for AetherWall, covering:
- Untrusted wallpaper packages
- HTML and JavaScript execution
- Network access
- Imported archives
- Interactive games
- Provider integrations
- Resource consumption
- Runtime crashes

## 2. Core principles

1. **No arbitrary native executables from wallpaper packages.** `.awall` and all
   imported archives must reject any entry that is a native executable, script,
   or dynamic library. There is no "trusted package" path.
2. **Untrusted content is sandboxed.** Web wallpapers and games run under the
   constrained HTML5 runtime with declared, permission-gated capabilities.
3. **Network access is opt-in and per-feature.** By default network, audio,
   mouse and keyboard input, file system read, and unlimited resources are all
   disabled unless the manifest explicitly declares them and the runtime policy
   allows them.
4. **The daemon never executes package payloads.** Package validation, extraction,
   and hash checks happen before any content reaches a renderer.
5. **The UI and the renderer are separate failure domains.** A crash in a
   wallpaper runtime must not crash Studio or the daemon.

## 3. Package security model (.awall)

### 3.1 Container

- `.awall` is a ZIP-compatible archive.
- Manifest: `wallpaper.json` (schema version 1.0).
- Only files needed by the selected wallpaper type are included.

### 3.2 Validation rules

Validation rejects:

| Rule | Reason |
|---|---|
| Path traversal (`../`, absolute paths, drive prefixes) | Prevent escape from package root |
| Symlinks | Prevent extraction outside package root |
| Archive entries escaping package root | Prevent overwrite of system files |
| Duplicate manifest keys | Prevent ambiguity |
| Unsupported schema versions | Prevent incompatibility |
| Archive exceeding size / expanded size / file count / compression ratio limits | Prevent ZIP bombs |
| Any native executable, script, or dynamic library entry | No arbitrary code execution |
| Referenced files that do not exist | Prevent broken packages |
| MIME/type inconsistencies where detectable | Prevent mislabeled payloads |

### 3.3 Installation lifecycle

1. Select archive.
2. Inspect metadata without extracting executable content.
3. Validate archive structure and manifest.
4. Show permissions, author, license, and size to the user.
5. Extract to a temporary directory.
6. Verify file hashes and referenced paths.
7. Atomically move into managed package storage.
8. Register in library database.
9. Generate or validate thumbnails.
10. Offer preview; do not auto-apply.

## 4. Runtime isolation

| Surface | Isolation | Enforced boundary |
|---|---|---|
| Web wallpaper | Constrained HTML5 runtime | Declared capabilities only; no arbitrary network, filesystem, or shell |
| Game | HTML5 game runtime | Stricter input and resource limits; no native payload |
| Anime/imported media | Native image/video runtime | No execution; only decode, render, preview |
| 3D scene | wgpu/glTF | GPU capability detection; no network or filesystem access |

## 5. Permission boundaries

### 5.1 Web wallpaper capabilities (declared in manifest)

| Capability | Default | Description |
|---|---|---|
| `network` | false | Network access for the web context |
| `audio` | false | Audio output |
| `mouse_input` | false | Mouse input |
| `keyboard_input` | false | Keyboard input |
| `filesystem_read` | false | Read access to file system |

### 5.2 Game capabilities (stricter)

| Capability | Default | Description |
|---|---|---|
| `mouse_input` | false | Mouse input |
| `keyboard_input` | false | Keyboard input |
| `save_state` | true | Persistent save-state namespace |
| `network` | false | Network access |
| `audio` | false | Audio output |

### 5.3 Resource limits (declared in manifest)

| Limit | Default | Description |
|---|---|---|
| `max_fps` | 30 | Maximum frames per second |
| `memory_limit_mb` | 512 | Max memory |
| `cpu_budget_percent` | 20 | Max CPU budget percent |

## 6. Provider integrations

- Online providers are adapters; provider-specific logic must not leak into
  the core domain model.
- Provider failure must not break the rest of the application.
- Provider failure shows provider-specific error; local library is preserved.
- Provider identity is metadata, not executable code.
- No DRM circumvention or scraping that violates provider terms.
- If provider authentication is ever added, credentials are stored in the
  desktop secret service/keyring, never in SQLite plaintext.

## 7. Resource consumption control

- Runtime resource limits from manifest are enforced where the runtime
  supports them.
- Web runtime memory use is capped by the HTML5 runtime.
- Video decode CPU/GPU usage is capped by the libmpv runtime.
- Image library indexing scales with library size (performance-tested).
- Pause/resume latency is bounded; heavy rendering pauses when hidden or
  covered by fullscreen where the compositor permits reliable detection.
- Multi-monitor scaling is measured and reported.

## 8. Runtime crash handling

| Failure domain | Isolation | Recovery |
|---|---|---|
| Studio UI | Separate from daemon and renderers | Reconnects to daemon; active sessions reconciled |
| Daemon | Per-user service | Studio reconnects; active sessions reconciled |
| Renderer/runtime | Separate process or thread | Daemon marks session failed, records diagnostics, offers retry/fallback |
| Provider | Separate adapter | Provider-specific error shown; local library preserved |
| Database | Separate file | Migration failure: stop safely, preserve previous database file |

## 9. Secrets and credentials

- Never store provider credentials in SQLite plaintext.
- Use the desktop secret service/keyring if provider authentication is ever
  added.
- Never log secrets or write them to logs or plaintext settings.

## 10. Threat model summary

| Threat | Mitigation |
|---|---|
| Path traversal in archive | Reject `../`, absolute paths, drive prefixes |
| ZIP bomb / resource exhaustion | Enforce archive size, expanded size, file count, compression ratio |
| Malicious web content | Sandboxed runtime; declared capabilities; permission gating |
| Malicious game | Constrained input/resource policy; no native executable payload |
| Malicious provider | Provider failure does not break the application |
| Credential theft | Secret service/keyring; no plaintext storage |
| Malicious archive content | Never execute shell scripts, binaries, or native libraries |
