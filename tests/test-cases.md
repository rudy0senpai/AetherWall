# Test Case Catalogue — AetherWall Phase 1

## Unit tests

| ID | Test | Expected result |
|---|---|---|
| T-UNIT-001 | Manifest parsing of valid `wallpaper.json` | Parses to typed structure |
| T-UNIT-002 | Manifest parsing of invalid JSON | Rejected with clear error |
| T-UNIT-003 | Manifest duplicate keys | Rejected |
| T-UNIT-004 | Unknown optional fields ignored | Ignored |
| T-UNIT-005 | Path normalization rejecting `../` traversal | Rejected |
| T-UNIT-006 | Path normalization rejecting absolute paths | Rejected |
| T-UNIT-007 | Playlist ordering with drag-and-drop | Order persists |
| T-UNIT-008 | Shuffle mode: sequential / shuffled / random | Shuffle seed deterministic |
| T-UNIT-009 | Settings serialization round-trip | Preserves values and types |
| T-UNIT-010 | Provider response normalization | Normalized; missing fields default |
| T-UNIT-011 | Display geometry mapping | Mapping correct for multi-monitor |
| T-UNIT-012 | Runtime state machine transitions | Allowed transitions only |

## Integration tests

| ID | Test | Expected result |
|---|---|---|
| T-INT-001 | Import local image | Image registered; thumbnail generated |
| T-INT-002 | Import local video | Video registered; metadata captured |
| T-INT-003 | Create and restore playlist | Playlist persisted and restored |
| T-INT-004 | Apply wallpaper to selected display | Wallpaper applied; state recorded |
| T-INT-005 | Restart daemon and reconcile sessions | Active sessions reconciled |
| T-INT-006 | Runtime crash and recovery | Daemon marks session failed; retry offered |
| T-INT-007 | Provider offline / error | Provider error shown; local library preserved |
| T-INT-008 | Package install / uninstall | Package registered and removable |
| T-INT-009 | Settings persistence | Settings survive daemon restart |

## Security tests

| ID | Test | Expected result |
|---|---|---|
| T-SEC-001 | Archive path traversal (`../`) | Rejected |
| T-SEC-002 | Archive absolute paths | Rejected |
| T-SEC-003 | Archive drive prefixes (Windows) | Rejected |
| T-SEC-004 | Symlink entry escaping root | Rejected |
| T-SEC-005 | ZIP bomb (large compression ratio) | Rejected |
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

## Manual validation

- Studio UI tested at 1440×900 and 1100×700
- Tested on available KDE Plasma session
- Loading, empty, error, offline states verified
- Keyboard and mouse interaction tested
- Display selection tested
- Preview/apply separation verified
