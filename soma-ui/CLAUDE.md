# CLAUDE.md

Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

## 5. Ponytail — Lazy Senior Dev Mode (always on)

**You are a lazy senior developer. Lazy means efficient, not careless. The best code is the code never written.**

Before writing any code, stop at the first rung that holds:

1. Does this need to be built at all? (YAGNI)
2. Does the standard library already do this? Use it.
3. Does a native platform feature cover it? Use it.
4. Does an already-installed dependency solve it? Use it.
5. Can this be one line? Make it one line.
6. Only then: write the minimum code that works.

Rules:

- No abstractions that weren't explicitly requested.
- No new dependency if it can be avoided.
- No boilerplate nobody asked for.
- Deletion over addition. Boring over clever. Fewest files possible.
- Question complex requests: "Do you actually need X, or does Y cover it?"
- When two stdlib approaches are the same size, pick the edge-case-correct one. Lazy means less code, not the flimsier algorithm.
- Mark intentional simplifications with a `ponytail:` comment. If the shortcut has a known ceiling (global lock, O(n²) scan, naive heuristic), the comment names the ceiling and the upgrade path.

**Not lazy about:** input validation at trust boundaries, error handling that prevents data loss, security, accessibility, the calibration real hardware needs (the platform is never the spec ideal — a clock drifts, a sensor reads off), and anything explicitly requested. Lazy code without its check is unfinished: non-trivial logic leaves ONE runnable check behind — the smallest thing that fails if the logic breaks (an assert-based demo/self-check or one small test file; no frameworks, no fixtures). Trivial one-liners need no test.

## 6. gstack — Automatic Skill Selection

Use gstack skills as needed — the system determines which to run from *what you're building*, without being told the skill name:

- **End-user products:** `/plan-design-review` (before) → `/design-review` (after)
- **Developer tools:** `/plan-devex-review` (before) → `/devex-review` (after)
- **Architecture:** `/plan-eng-review` (before) → `/review` (after)
- **Everything:** `/autoplan` auto-detects the applicable reviews and surfaces only taste decisions needing approval.

Other gstack skills (auto-routed by intent): `/office-hours`, `/spec`, `/design-shotgun`, `/design-html`, `/qa`, `/investigate`, `/ship`, `/land-and-deploy`.

## 7. Global Rules (always apply)

The global rules in `~/.claude/CLAUDE.md` and their skills apply to every change in this repo — they are the source of truth, do not duplicate them here:

- **Rust — `/rust-skills`**: 179 rules across 14 categories (ownership, error handling, async, API design, memory, performance, testing, anti-patterns). ALL Rust written, reviewed, or refactored here must follow these. Consult before and during any Rust work.
- **Ponytail** (§5): the lazy-senior-dev ladder for every line; review the diff with `/ponytail-review` and the repo with `/ponytail-audit` after building.
- **gstack workflow** (§6): plan review up front for non-trivial features, `/review` before a PR, `/design-review` for UI.
- **db-standards — `/db-standards`**: if/when this project talks to a database.
- **humanizer — `/humanizer`**: applied to any user-facing prose or narration.

## 8. This Project — soma-ui

**soma-ui is a single cross-platform UI / design-system monorepo** — one home for the same component library across targets: **web** (Leptos, today), **mobile** (Flutter, today), **native** (later). The design language is identical everywhere: Palantir-style **slate/blue, light + dark**, Outfit (body) + Rajdhani (headings). Built copy-paste-first (à la rust-ui.com — you own the source, no black-box crates), so components are reusable across apps.

### Web — Leptos (CSR)

- **`web/packages/ui`** (`soma-ui` crate) — Leptos components styled with Tailwind CSS; light + dark via semantic CSS-variable tokens (defined in `web/playground/style/main.css`). Every component/page responsive.
- **Component layout:** grouped by **category folder** under `web/packages/ui/src/components/<category>/` (`inputs/`, `data_display/`, `layout/`, `motion/`, …). Each category folder holds the component files (`button.rs`, …) + a `mod.rs` that declares + re-exports them; `components/mod.rs` re-exports each category. Public API stays flat (`soma_ui::Button`). New components go in the matching category; a new category folder only for a genuinely new group. No one-folder-per-component, no flat files directly in `components/`. (Charts, blocks, screens, and hooks live in their own top-level modules under `web/packages/ui/src/`.)
- **`web/playground/`** — Leptos CSR app (Trunk + `wasm32-unknown-unknown`): sidebar nav, one route per component, live preview + signal-driven controls, plus a viewport toolbar for responsive testing. Page boilerplate is shared via `web/playground/src/ui.rs` (`PageShell`/`PreviewPanel`/`ControlsPanel`/`ControlRow`).
- **Verify:** `cd web && cargo check --target wasm32-unknown-unknown -p playground`. **Run:** `./dev.sh` (serves the playground on port 9876; workspace lives in `web/`).

### Mobile — Flutter

- **`flutter/`** — a pure-Dart Flutter package (`soma_ui`) mirroring the web components' APIs, theme, **and folder structure**. Palantir tokens ported from the web `main.css` into `flutter/lib/src/theme/`; Outfit/Rajdhani bundled as self-hosted TTF in `flutter/assets/fonts/`.
- **Component layout (mirrors web):** grouped by **category folder** under `flutter/lib/src/components/<category>/` — the SAME categories as the web crate (`inputs/`, `data_display/`, `feedback/`, `navigation/`, `overlays/`, `layout/`, `forms/`, `disclosure/`, `motion/`, `interaction/`, …). Each category folder holds the widget files (`soma_button.dart`, …) + a barrel `<category>.dart` (Dart's analog of `mod.rs`) that re-exports them; `components/components.dart` re-exports each category barrel; `lib/soma_ui.dart` re-exports theme + icons + components. Public API stays flat (`import 'package:soma_ui/soma_ui.dart'; SomaButton(...)`). A new widget goes in the category matching its web counterpart; a new category folder only for a genuinely new group. **No flat widget files directly in `components/`.**
- **Icons:** Lucide, for parity with the web crate's `icondata` (lucide feature). Package `lucide_icons_flutter`, re-exported via `lib/src/icons/soma_icons.dart`; reference as `Icon(LucideIcons.chevronDown, …)`.
- **Theme:** light + dark via `SomaThemeProvider`/`SomaTheme` (InheritedWidget) over `SomaColors.light`/`.dark`; `SomaThemeToggle` (in `interaction/`) is the reusable light/dark switch, wired into the example app's app bar.
- **`flutter/example/`** — playground gallery: sidebar nav grouped by the same categories, one screen per component (`example/lib/screens/<name>_screen.dart`, flat) registered in `example/lib/main.dart` `_navEntries`; each screen uses the shared `ComponentPage` (preview + live controls).
- **Verify:** `flutter analyze` (+ `flutter build web`). **Run gallery:** `cd flutter/example && flutter run -d chrome`.

### Cross-platform rules

- **Latest stable everywhere** (deps AND toolchains): `cargo add` / `flutter upgrade` / `flutter pub upgrade --major-versions`. Never hardcode a version number in this file — let the lockfiles be the source of truth.
- **Keep the design in sync:** token VALUES and component APIs (variants/sizes) must match across web and Flutter. The web `web/playground/style/main.css` token block is the source of truth; port any change into the Flutter theme.

### Shared components — soma-ui IS the UI layer

Platform-wide rule: `../CLAUDE.md` ("Shared components"). soma-ui is the **producer** of the shared UI layer, the front-end counterpart to `soma-infra` (backend plumbing):

- **All reusable UI components live here.** Every soma service consumes soma-ui for buttons/inputs/charts/layouts rather than rebuilding them. When a service needs a component that doesn't exist yet, add it here (generic, themeable) — don't fork a one-off into the service.
- **soma-ui holds UI mechanism, not service policy.** Components are presentation primitives; what they *do* on click, what data they show, and any business rules stay in the consuming service.
- If soma-ui ever grows backend code (build tooling, a token-sync service, etc.), that plumbing comes from `soma-infra` — same rule as every other service: don't hand-roll a pool/telemetry/crypto/HTTP client.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.