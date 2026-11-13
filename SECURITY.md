# Security Policy

## 1. Scope of this policy

AetherWall loads and runs third-party artwork and authored content. This policy
covers the security boundaries that Phase 0 establishes for that untrusted content
and for the AetherWall process itself.

## 2. Core principles

1. **No arbitrary native executables from wallpaper packages.** `.awall` packages
   and all other imported archives must reject any entry that is a native
   executable, script, or dynamic library. There is no "trusted package" path.
2. **Untrusted content is sandboxed.** Web wallpapers and games run under the
   constrained HTML5 runtime with declared, permission-gated capabilities.
3. **Network access is opt-in and per-feature.** By default network, audio, mouse
   and keyboard input, file system read, and unlimited resources are all disabled
   unless the manifest explicitly declares them and the runtime policy allows them.
4. **The daemon never executes package payloads.** Package validation, extraction,
   and hash checks happen before any content reaches a renderer.
5. **The UI and the renderer are separate failure domains.** A crash in a
   wallpaper runtime must not crash Studio or the daemon.

## 3. Package security model

- `.awall` is a ZIP-compatible archive with a versioned JSON manifest (`wallpaper.json`).
- Validation rejects:
  - Path traversal (`../`, absolute paths, drive prefixes).
  - Symlinks and any archive entry that escapes the package root.
  - Duplicate manifest keys and unsupported schema versions.
  - Any archive exceeding configured size, file count, or compression-ratio limits.
- Extracted content is only registered in the library database after hash and
  referenced-path verification.
- Never execute shell scripts, binaries, or native libraries from a package.

## 4. Runtime isolation

| Surface | Isolation | Enforced boundary |
|---|---|---|
| Web wallpaper | Constrained HTML5 runtime | Declared capabilities only; no arbitrary network, filesystem, or shell |
| Game | HTML5 game runtime | Stricter input and resource limits; no native payload |
| Anime/imported media | Native image/video runtime | No execution; only decode, render, preview |
| 3D scene | wgpu/glTF | GPU capability detection; no network or filesystem access |

## 5. Secrets and credentials

- Never store provider credentials in SQLite plaintext.
- Use the desktop secret service/keyring if provider authentication is ever added.
- Never log secrets or write them to logs or plaintext settings.

## 6. Threat model summary

| Threat | Mitigation |
|---|---|
| Path traversal in archive | Reject `../`, absolute paths, drive prefixes |
| ZIP bomb / resource exhaustion | Enforce archive size, expanded size, file count, compression ratio |
| Malicious web content | Sandboxed runtime; declared capabilities; permission gating |
| Malicious game | Constrained input/resource policy; no native executable payload |
| Malicious provider | Provider failure must not break the application |
| Credential theft | Secret service/keyring; no plaintext storage |

## 7. Reporting a security issue

Please do not open a public issue. Report to the security contact listed in
`SECURITY.md` (or via the repository's private security advisory). Include the
affected version, reproduction steps, and the asset/URL involved.
