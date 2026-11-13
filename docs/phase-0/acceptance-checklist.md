# Phase 0 Acceptance Checklist

## Product definition
- [ ] Product vision and target users documented.
- [ ] Eight wallpaper categories defined.
- [ ] Functional and non-functional requirements documented.
- [ ] Explicit exclusions documented, including removal of system-monitoring HUD from the new UI.

## Repository and workflow
- [ ] Proposed repository tree reviewed.
- [ ] Legacy v4 preservation strategy documented.
- [ ] Coding-agent task protocol documented.
- [ ] Branch, commit and review conventions documented.

## Architecture
- [ ] Qt/QML Studio boundary documented.
- [ ] Rust daemon/core boundary documented.
- [ ] Hybrid runtime responsibilities documented.
- [ ] KDE Plasma 6 Wayland-first integration strategy documented.
- [ ] Runtime lifecycle and crash isolation documented.
- [ ] IPC contract outline documented.

## UI/UX
- [ ] Top desktop preview and lower Studio layout prototyped.
- [ ] Library and Workshop distinction prototyped.
- [ ] All eight category screens represented.
- [ ] Wallpaper details/customization flow represented.
- [ ] Loading, empty, error and offline states represented.
- [ ] Minimum window size and scaling reviewed.
- [ ] No system-monitoring HUD appears in the design.

## Data and package
- [ ] Logical SQLite schema reviewed.
- [ ] `.awall` manifest and package layout reviewed.
- [ ] Package validation and permission model reviewed.
- [ ] Provider attribution and licensing fields included.

## Quality and security
- [ ] Test strategy reviewed.
- [ ] Web/game sandbox requirements documented.
- [ ] Archive extraction threat model reviewed.
- [ ] Performance and platform test matrix documented.
- [ ] Open questions and unsupported capabilities listed.

## Exit approval
- [ ] Product owner review complete.
- [ ] Technical architecture review complete.
- [ ] UI/UX review complete.
- [ ] Phase 0 exit decision recorded.
- [ ] Phase 1 scope authorized separately.
