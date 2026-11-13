# Testing and Validation Strategy

## 1. Phase 0 validation

Phase 0 validates the design and contracts, not production playback.

Required checks:
- Documentation consistency review.
- Repository tree review.
- UI navigation and screen coverage review.
- Package schema review with valid and invalid fixtures.
- Data model relationship and migration review.
- Runtime lifecycle contract review.
- Threat-model review for imported packages and web/game content.
- Prototype walkthrough at reference and minimum window sizes.

## 2. Future automated test layers

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

## 3. Platform test matrix

| Environment | Priority | Validation |
|---|---|---|
| KDE Plasma 6 Wayland, NVIDIA | P0/P1 | Primary target |
| KDE Plasma 6 Wayland, Intel iGPU | P1 | Low-power compatibility |
| KDE Plasma 6 X11 | P1 | Compatibility backend |
| KDE Plasma 6 Wayland, AMD | P2 | Hardware compatibility |
| Other Wayland compositors | Later | Capability-specific support |

## 4. Manual test report template

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

## 5. Release gate

Do not label a phase complete based solely on compilation. Require relevant automated tests, manual desktop validation and documented limitations.
