# Development Workflow and Coding-Agent Rules

## 1. General workflow

- Work in small, reviewable phases.
- Read the relevant specification before changing a module.
- Inspect the current repository and working tree before editing.
- Do not overwrite legacy v4 code or assets without an explicit migration task.
- Do not claim a feature works without running the relevant test.
- Record commands, results, failures and untested areas.
- Keep documentation and implementation changes in the same pull request when behaviour or contracts change.

## 2. Branch and commit conventions

Suggested branches:
- `phase-0/*`
- `phase-1/*`
- `feature/<short-name>`
- `fix/<short-name>`
- `docs/<short-name>`

Suggested commit format:
- `docs: define Phase 0 architecture`
- `feat(library): add wallpaper indexing`
- `fix(runtime): recover after mpv exit`
- `test(package): reject traversal paths`

Do not push directly to the default branch for feature work.

## 3. Coding standards

### Rust
- Format with `cargo fmt`.
- Lint with `cargo clippy --all-targets --all-features -- -D warnings` where dependencies permit.
- Prefer typed errors and structured diagnostics.
- Avoid blocking I/O on async executor threads.
- Keep runtime adapters behind traits/interfaces.
- Do not use `unsafe` without a documented justification and review.

### Qt/QML
- Keep QML presentation-focused.
- Put domain operations behind typed service/controller boundaries.
- Never block the UI thread on media, network or filesystem work.
- Use reusable components and centralized design tokens.
- Provide loading, empty, error and disabled states.
- Keep accessibility labels and keyboard focus order.

### Documentation
- Use Markdown.
- Define acronyms on first use.
- Keep diagrams and API contracts versioned.
- Mark assumptions and unresolved questions explicitly.
- Do not document proposed functionality as implemented.

## 4. Agent task protocol

Every coding-agent task must include:
1. Goal and scope
2. Files/modules allowed to change
3. Files/modules explicitly out of scope
4. Relevant specification references
5. Acceptance criteria
6. Required tests
7. Validation commands
8. Expected report format

Agent completion report must include:
- Summary
- Files changed
- Tests executed and results
- Manual checks performed
- Known limitations
- Follow-up work
- Confirmation that unrelated files were not changed

## 5. Definition of done

A task is done only when:
- Acceptance criteria are met.
- Automated tests pass or failures are explicitly documented.
- Formatting/lint checks pass where applicable.
- UI states and error paths are handled.
- Documentation is updated.
- No unrelated changes are present.
- No unsupported success claims are made.
