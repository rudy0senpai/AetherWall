# Prototype Specification — AetherWall Phase 0

## Scope

Interactive HTML/CSS prototype covering the 16 required screens. Built from the
AetherWall design system tokens (`design/design-system/tokens.css`,
`typography.css`, `spacing.css`).

## Validation approach

- Prototype walkthrough at reference size 1440×900 and minimum 1100×700.
- Verify:
  - Desktop preview is top, Studio below.
  - No system-monitoring HUD.
  - All 8 categories present in nav and in category chips.
  - All 16 screens accessible.
  - Components use design tokens.
  - Loading, empty, error, offline states present.
  - Web editor has editor/preview split; 3D editor has viewport/hierarchy/
    properties.

## Limitations

- No production rendering; media is represented by placeholders.
- No real Qt/QML runtime.
- Prototypes are design references, not implementation.

## How to run

Open `design/prototypes/prototype.html` in a modern browser. Use the sidebar
nav to jump between screens. Some screens (e.g. Video, Web Editor, 3D) include
sample content that does not require a backend.
