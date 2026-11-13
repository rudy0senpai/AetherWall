# AGENTS.md — AetherWall Phase 0 Foundation

This file is the project-wide coding-agent instruction set for AetherWall.

## 1. Project identity

- **Product:** AetherWall — a Linux-native wallpaper platform for KDE Plasma 6.
- **First-class desktop:** KDE Plasma 6, Wayland first; X11 compatibility where feasible.
- **UI:** Qt 6 / QML.
- **Core:** Rust background service (`aetherwall-daemon`) and shared domain crates (`crates/`).
- **Rendering:** Hybrid runtimes — native image, `libmpv`, Qt WebEngine, `wgpu` + glTF, constrained HTML5 game runtime.
- **Persistence:** SQLite.
- **IPC:** D-Bus with versioned message contracts.
- **Initial scope:** All eight wallpaper categories, delivered incrementally.

## 2. Repository hygiene

1. Work on isolated branches (`phase-0/*`, `phase-1/*`, `feature/<name>`, `fix/<name>`, `docs/<name>`). Never commit directly to `main`.
2. Commit message format: `<type>(<scope>): <summary>` where type is `docs|feat|fix|test|refactor|chore`.
3. Inspect the working tree and git status before every edit. Never overwrite existing files blindly.
4. Preserve legacy v4. The legacy v4 implementation is stored in git history (`v4.1` commit `b4afa86`, `Initial commit` `008a4fc`). Do not delete, rename, or rewrite its HUD, telemetry, Plasma plugins, or installer unless an explicit migration task approves it.
5. Keep documentation and implementation changes in the same pull request when behaviour or contracts change.
6. Do not introduce production wallpaper playback code during Phase 0. Phase 0 is design + specification + prototype only.

## 3. Coding standards

### Rust
- Format with `cargo fmt --all`.
- Lint with `cargo clippy --all-targets --all-features -- -D warnings` where dependencies permit.
- Prefer typed errors and structured diagnostics; avoid blocking I/O on async executor threads.
- Keep runtime adapters behind traits/interfaces.
- Do not use `unsafe` without a documented justification and review.

### Qt/QML
- Keep QML presentation-focused; domain operations go behind typed service/controller boundaries.
- Never block the UI thread on media, network, or filesystem work.
- Use reusable components and centralized design tokens from `design/design-system/`.
- Provide loading, empty, error, and disabled states on every component.
- Keep accessibility labels and keyboard focus order.

### Documentation
- Use Markdown. Define acronyms on first use.
- Keep diagrams and API contracts versioned.
- Mark assumptions and unresolved questions explicitly.
- Do not document proposed functionality as implemented.

## 4. Agent task protocol

Every coding-agent task must include:
1. Goal and scope.
2. Files/modules allowed to change.
3. Files/modules explicitly out of scope.
4. Relevant specification references (`docs/phase-0/*.md`).
5. Acceptance criteria.
6. Required tests.
7. Validation commands.
8. Expected report format.

Agent completion report must include:
- Summary, files changed, tests executed and results, manual checks performed, known limitations, follow-up work, confirmation that unrelated files were not changed.

## 5. Definition of done

A task is done only when:
- Acceptance criteria are met.
- Automated tests pass or failures are explicitly documented.
- Formatting/lint checks pass where applicable.
- UI states and error paths are handled.
- Documentation is updated.
- No unrelated changes are present.
- No unsupported success claims are made.

## 6. Phase 0 scope boundary

Phase 0 covers:
- Repository structure, documentation, design system, screen prototypes, architecture decisions, security model, test strategy, and validation.
- It does **not** implement production wallpaper playback, Rust core, or Qt/QML Studio code.

Do not begin Phase 1 automatically. Phase 1 scope is authorized separately.
