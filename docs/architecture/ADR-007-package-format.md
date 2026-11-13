# ADR-007 — Package format is declarative

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

AetherWall must distribute wallpapers in a portable, validated package format.

## Decision

The `.awall` package is a **ZIP-compatible archive** with a **versioned JSON
manifest** (`wallpaper.json`). **Native executable payloads are prohibited.**

## Reasons

- Portability and validation.
- Reduced supply-chain risk: the package cannot contain native executables.
- JSON is readable, versionable, and easy to validate against a schema.

## Consequences

- Validation rejects:
  - Path traversal (`../`, absolute paths, drive prefixes).
  - Symlinks and any archive entry escaping the package root.
  - Duplicate manifest keys and unsupported schema versions.
  - Any archive exceeding size, file count, or compression-ratio limits.
- Extracted content is only registered after hash and referenced-path
  verification.
- Never execute shell scripts, binaries, or native libraries from a package.
- Treat HTML and JavaScript as untrusted content.
