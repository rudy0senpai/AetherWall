# Contributing to AetherWall

Thank you for wanting to contribute. AetherWall is in early design; before you start, read:

- [README.md](README.md)
- [docs/phase-0/README.md](docs/phase-0/README.md)
- [docs/phase-0/development-workflow.md](docs/phase-0/development-workflow.md)
- [AGENTS.md](AGENTS.md)

## 1. Ways to contribute

- **Documentation:** Improve specs, diagrams, and the checklist in `docs/phase-0/`.
- **Design:** Provide feedback on the design system and screen prototypes.
- **Architecture:** Review ADRs and IPC contract outlines.
- **Prototypes:** Build or improve HTML/CSS screen prototypes in `design/prototypes/`.
- **Testing:** Propose test cases for the catalogue in `tests/`.

## 2. Workflow

1. Create a branch: `git checkout -b phase-0/<short-name>` (or `feature/`, `fix/`, `docs/`).
2. Make focused, reviewable changes. Do not mix unrelated work in one commit.
3. Run the relevant validation after every significant change.
4. Open a pull request against `main`. Include the Phase 0 acceptance checklist status.

## 3. Commit and branch conventions

- Branch names: `phase-0/<name>`, `phase-1/<name>`, `feature/<name>`, `fix/<name>`, `docs/<name>`.
- Commit messages: `<type>(<scope>): <summary>` with type in `docs|feat|fix|test|refactor|chore`.

## 4. What not to do

- Do not modify legacy v4 without an explicit migration task. Legacy v4 is preserved in git history.
- Do not push directly to `main` for feature work.
- Do not claim unimplemented features are complete.
