# Test Directory — AetherWall

The `tests/` directory holds test layers and test fixtures.

## Structure

| Directory | Purpose |
|---|---|
| `unit/` | Unit tests for crates (Phase 1) |
| `integration/` | Cross-module integration tests (Phase 1) |
| `contract/` | D-Bus request/response, runtime lifecycle, provider interface, package manifest compatibility |
| `security/` | Archive path traversal, symlink extraction, ZIP bomb limits, malformed manifest, web permission enforcement, game resource limits, secrets |
| `performance/` | Idle daemon resource usage, image library indexing at scale, video decode usage, web runtime memory, 3D frame pacing/GPU fallback, pause/resume latency, multi-monitor scaling |

## Future automated test layers (Phase 1+)

### Unit tests
- Manifest parsing and validation.
- Path normalization and traversal rejection.
- Playlist ordering and shuffle logic.
- Settings serialization.
- Provider response normalization.
- Display geometry mapping.
- Runtime state machine.

### Contract tests
- D-Bus request/response schemas.
- Runtime adapter lifecycle.
- Provider adapter interface.
- Package manifest compatibility.

### Integration tests
- Import local image and video.
- Create and restore playlist.
- Apply wallpaper to selected display.
- Restart daemon and reconcile sessions.
- Runtime crash and recovery.
- Provider offline/error behaviour.
- Package install/uninstall.
- Settings persistence.

### UI tests
- Navigation and focus order.
- Import/download progress and cancellation.
- Preview/apply separation.
- Empty/loading/error states.
- Display selection and responsive layout.
- Accessibility labels.

### Security tests
- Archive path traversal.
- Symlink extraction.
- ZIP bomb limits.
- Malformed manifest and duplicate keys.
- Web permission enforcement.
- Network restrictions.
- Game runtime resource limits.
- Secrets not written to logs or plaintext settings.

### Performance tests
- Idle daemon resource usage.
- Image library indexing at increasing library sizes.
- Video decode CPU/GPU usage.
- Web runtime memory use.
- 3D frame pacing and GPU fallback.
- Pause/resume latency.
- Multi-monitor scaling.

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
