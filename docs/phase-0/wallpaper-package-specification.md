# AetherWall Wallpaper Package Specification

**Proposed extension:** `.awall`  
**Container:** ZIP-compatible archive  
**Manifest:** `wallpaper.json`  
**Schema version:** `1.0`

## 1. Goals

- Portable package for user-created and shared wallpapers.
- Explicit media type and runtime requirements.
- No arbitrary native executable payloads.
- Deterministic validation before installation.
- Permission declarations and source attribution.
- Forward-compatible schema versioning.

## 2. Package layout

```text
example.awall
├── wallpaper.json
├── preview.png
├── thumbnail.webp
├── assets/
│   ├── background.webp
│   └── audio/
└── content/
    ├── index.html
    ├── style.css
    ├── script.js
    └── scene.glb
```

Only files needed by the selected wallpaper type should be included.

## 3. Manifest example

```json
{
  "schema_version": "1.0",
  "id": "org.example.aetherwall.nebula",
  "name": "Nebula Drift",
  "version": "1.0.0",
  "description": "A slowly animated nebula scene.",
  "type": "web",
  "entrypoint": "content/index.html",
  "preview": "preview.png",
  "thumbnail": "thumbnail.webp",
  "author": {
    "name": "Example Author",
    "url": "https://example.invalid"
  },
  "license": {
    "id": "CC-BY-4.0",
    "url": "https://creativecommons.org/licenses/by/4.0/"
  },
  "runtime": {
    "id": "web",
    "minimum_version": "1.0"
  },
  "capabilities": {
    "network": false,
    "audio": false,
    "mouse_input": true,
    "keyboard_input": false,
    "filesystem_read": false
  },
  "resources": {
    "max_fps": 30,
    "memory_limit_mb": 512,
    "cpu_budget_percent": 20
  },
  "tags": ["space", "abstract", "animated"]
}
```

The example is illustrative. The final JSON Schema must define required fields, allowed enums, limits, path rules and compatibility behaviour.

## 4. Type-specific requirements

- `image`: one or more supported image assets.
- `slideshow`: playlist or collection references; not a separate executable runtime.
- `video`: media asset and optional playback defaults.
- `web`: HTML entrypoint and declared permissions.
- `anime-image`: imported media plus provider/source attribution; online provider identity is metadata, not executable code.
- `anime-video`: imported media plus provider/source attribution.
- `3d`: scene entrypoint, supported glTF/GLB assets and render defaults.
- `game`: HTML5 entrypoint, declared input permissions, save-state namespace and stricter resource limits.

## 5. Validation requirements

- Reject path traversal (`../`, absolute paths, drive prefixes).
- Reject symlinks and archive entries escaping package root.
- Enforce maximum archive size, expanded size, file count and compression ratio.
- Reject duplicate manifest keys and unsupported schema versions.
- Validate all referenced files exist.
- Verify MIME/type consistency where possible.
- Never execute shell scripts, binaries or native libraries from a package.
- Treat HTML and JavaScript as untrusted content.
- Validate declared permissions against runtime policy.
- Record package hash for integrity and diagnostics.

## 6. Installation lifecycle

1. Select archive.
2. Inspect metadata without extracting executable content.
3. Validate archive structure and manifest.
4. Show permissions, author, license and size.
5. Extract to a temporary directory.
6. Verify file hashes and referenced paths.
7. Atomically move into managed package storage.
8. Register in library database.
9. Generate or validate thumbnails.
10. Offer preview; do not auto-apply.

## 7. Compatibility

- Unknown optional fields are ignored.
- Unknown required capabilities cause a clear incompatibility error.
- Major schema incompatibility is rejected.
- Package upgrades must not silently overwrite user settings or save data.
