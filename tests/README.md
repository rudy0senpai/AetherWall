# Test Directory — AetherWall Phase 1

The `tests/` directory holds test suites for the Phase 1 implementation.

## Structure

| Directory | Purpose |
|---|---|
| `unit/` | Unit tests for Rust crates (core, library, scheduler) |
| `integration/` | Cross-module tests (daemon + library + IPC) |
| `contract/` | D-Bus request/response, runtime adapter interface |
| `security/` | Path traversal, archive validation, package manifest |
| `performance/` | Database performance, startup time, memory usage |

## Test layers

### Unit tests
- Manifest parsing and validation
- Path normalization and traversal rejection
- Playlist ordering and shuffle logic
- Settings serialization
- Display geometry mapping
- Runtime state machine

### Integration tests
- Import local image
- Create and restore playlist
- Apply wallpaper to selected display
- Restart daemon and reconcile sessions
- Runtime crash and recovery
- Settings persistence

### Contract tests
- D-Bus request/response schemas
- Runtime adapter lifecycle (via mock runtime)
- Provider adapter interface
- Package manifest compatibility

### Security tests
- Archive path traversal
- Symlink extraction
- ZIP bomb limits
- Malformed manifest
- Web permission enforcement

## Run tests

```bash
# Run all workspace tests
cargo test --workspace

# Run a specific crate's tests
cargo test -p aetherwall-core
cargo test -p aetherwall-library
cargo test -p aetherwall-daemon

# Run a specific test
cargo test -p aetherwall-core -- test_session_transitions
```

## Manual test report template

For each test, record:
- Test ID
- OS and kernel
- Plasma version
- Session type
- GPU and driver
- Display topology
- Steps
- Expected result
- Actual result
- Logs/artifacts
- Pass/fail/blocked
- Reproduction notes
