# Test Case Catalogue — AetherWall Phase 0

This catalogue covers the Phase 0 acceptance criteria and the future test layers
defined in `docs/phase-0/testing-and-validation.md`.

## Test categories

| ID | Category | Test ID prefix |
|---|---|---|
| Unit | Manifest parsing, path normalization, playlist logic, settings, provider normalization, display geometry, runtime state machine | `T-UNIT-00x` |
| Contract | D-Bus schemas, runtime lifecycle, provider interface, manifest compatibility | `T-CON-00x` |
| Integration | Import, playlists, apply, restart/reconcile, crash/recovery, provider offline, package install/uninstall, settings persistence | `T-INT-00x` |
| UI | Navigation, progress/cancel, preview/apply separation, empty/loading/error, display selection, accessibility | `T-UI-00x` |
| Security | Path traversal, symlink, ZIP bomb, malformed manifest, web permissions, network restrictions, game limits, secrets | `T-SEC-00x` |
| Performance | Idle daemon, library indexing, video decode, web memory, 3D frame pacing, pause/resume latency, multi-monitor scaling | `T-PERF-00x` |

---

## Unit tests

| ID | Test | Expected result |
|---|---|---|
| T-UNIT-001 | Manifest parsing of valid `wallpaper.json` | Parses to typed structure; manifest version 1.0 accepted |
| T-UNIT-002 | Manifest parsing of invalid JSON | Rejected with clear error |
| T-UNIT-003 | Manifest duplicate keys | Rejected |
| T-UNIT-004 | Unknown optional fields ignored | Ignored; no error |
| T-UNIT-005 | Path normalization that rejects `../` traversal | Rejected |
| T-UNIT-006 | Path normalization that rejects absolute paths and drive prefixes | Rejected |
| T-UNIT-007 | Playlist ordering with drag-and-drop | Order persists; position updates |
| T-UNIT-008 | Shuffle mode: sequential / shuffled / random | Shuffle seed deterministic for reproducibility |
| T-UNIT-009 | Settings serialization round-trip | Preserves values and types |
| T-UNIT-010 | Provider response normalization | Normalized; missing fields default or omitted |
| T-UNIT-011 | Display geometry mapping (logical x/y/width/height, scale) | Mapping correct for multi-monitor topology |
| T-UNIT-012 | Runtime state machine transitions | Allowed transitions only; invalid rejected |

## Contract tests

| ID | Test | Expected result |
|---|---|---|
| T-CON-001 | D-Bus request/response schema validation | Valid requests accepted; invalid rejected with error code |
| T-CON-002 | Runtime lifecycle: start → pause → resume → stop | All transitions succeed; state recorded |
| T-CON-003 | Runtime lifecycle invalid transitions | Rejected with structured error |
| T-CON-004 | Provider adapter interface contract | Adapter conforms to provider interface |
| T-CON-005 | Package manifest compatibility | V1.0 manifest accepted; V2.0 rejected without breakage of V1.0 |
| T-CON-006 | Package manifest with unknown required capability | Rejected with clear incompatibility error |

## Integration tests

| ID | Test | Expected result |
|---|---|---|
| T-INT-001 | Import local image | Image registered in library; thumbnail generated |
| T-INT-002 | Import local video | Video registered; metadata (duration, resolution, fps) captured |
| T-INT-003 | Create and restore playlist | Playlist persisted and restored after session restart |
| T-INT-004 | Apply wallpaper to selected display | Wallpaper applied; state recorded |
| T-INT-005 | Restart daemon and reconcile sessions | Active sessions reconciled; state preserved |
| T-INT-006 | Runtime crash and recovery | Daemon marks session failed, records diagnostics, offers retry |
| T-INT-007 | Provider offline / error | Provider error shown; local library preserved |
| T-INT-008 | Package install / uninstall | Package registered and removable |
| T-INT-009 | Settings persistence | Settings survive daemon restart |

## UI tests

| ID | Test | Expected result |
|---|---|---|
| T-UI-001 | Navigation focus order | Logical focus order; visible focus ring |
| T-UI-002 | Preview/apply separation | Selecting a wallpaper does not auto-apply; Apply opens display selection |
| T-UI-003 | Import/download progress and cancellation | Progress shown; cancel allowed |
| T-UI-004 | Empty state | Correct empty state shown for each list |
| T-UI-005 | Loading / error / offline states | Correct states shown for each condition |
| T-UI-006 | Display selection | Display selector lists displays; apply applies to chosen |
| T-UI-007 | Responsive layout (resize window) | Media grid adapts columns; navigation collapses |
| T-UI-008 | Accessibility labels | All interactive elements have labels |

## Security tests

| ID | Test | Expected result |
|---|---|---|
| T-SEC-001 | Archive path traversal (`../`) | Rejected |
| T-SEC-002 | Archive absolute paths | Rejected |
| T-SEC-003 | Archive drive prefixes (Windows) | Rejected |
| T-SEC-004 | Symlink entry escaping root | Rejected |
| T-SEC-005 | ZIP bomb (large compression ratio) | Rejected with size limit error |
| T-SEC-006 | Malformed manifest | Rejected |
| T-SEC-007 | Duplicate manifest keys | Rejected |
| T-SEC-008 | Unsupported schema version | Rejected |
| T-SEC-009 | Web permission `network` false | Network blocked |
| T-SEC-010 | Web permission `filesystem_read` false | No filesystem read |
| T-SEC-011 | Game resource limit `max_fps` | FPS capped |
| T-SEC-012 | Game resource limit `memory_limit_mb` | Memory capped |
| T-SEC-013 | Native executable entry in package | Rejected |
| T-SEC-014 | Secrets not in logs | No credentials in logs |
| T-SEC-015 | Secrets not in plaintext settings | Stored in keyring only |

## Performance tests

| ID | Test | Expected result |
|---|---|---|
| T-PERF-001 | Idle daemon resource usage | CPU/RAM within baseline; documented |
| T-PERF-002 | Image library indexing at scale | Indexing completes within bound at 10k, 100k items |
| T-PERF-003 | Video decode CPU/GPU usage | Within documented bounds |
| T-PERF-004 | Web runtime memory use | Within `memory_limit_mb` |
| T-PERF-005 | 3D frame pacing / GPU fallback | Frame pacing acceptable; GPU fallback triggers honestly |
| T-PERF-006 | Pause/resume latency | Latency within bound |
| T-PERF-007 | Multi-monitor scaling | Scaling correct for each display |

---

## Phase 0 validation

The Phase 0 validation does **not** run these automated tests (production
playback code is out of scope). It validates:

- Documentation consistency review.
- Repository tree review.
- UI navigation and screen coverage review.
- Package schema review with valid and invalid fixtures.
- Data model relationship and migration review.
- Runtime lifecycle contract review.
- Threat-model review for imported packages and web/game content.
- Prototype walkthrough at reference and minimum window sizes.
