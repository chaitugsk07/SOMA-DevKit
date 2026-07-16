# Contributing to soma-ui

Contributions are welcome — new components, bug fixes, token changes, or web/Flutter parity improvements.

---

## Prerequisites

**Web (Leptos)**
- Rust stable toolchain
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`

**Flutter**
- Flutter stable channel

---

## Dev Setup

**Web playground** (hot-reload on `:9876`):
```sh
./dev.sh
```

**Flutter gallery**:
```sh
cd flutter/example && flutter run -d chrome
```

---

## Where Components Live

**Web:**
```
web/packages/ui/src/components/<category>/
  <component>.rs        # component implementation
  mod.rs                # declares and re-exports all files in the category
```
`web/packages/ui/src/components/mod.rs` re-exports each category. The public API stays flat: `use soma_ui::Button`.

**Flutter:**
```
flutter/lib/src/components/<category>/
  soma_<component>.dart  # widget implementation
  <category>.dart        # barrel file, re-exports all widgets in the category
```
`flutter/lib/src/components/components.dart` re-exports each category barrel. Public API: `import 'package:soma_ui/soma_ui.dart'; SomaButton(...)`.

The same categories exist on both targets. New components belong in the matching category folder.

---

## Adding a New Component

Work through both targets together so they ship in parity:

**Web checklist:**
- [ ] Add `<component>.rs` in the appropriate `web/packages/ui/src/components/<category>/` folder
- [ ] Export it from that folder's `mod.rs`
- [ ] Add a playground page: `web/playground/src/pages/<component>.rs` + register the route in `web/playground/src/main.rs`
- [ ] Use CSS custom property tokens (e.g. `var(--color-primary)`) — no hardcoded color values

**Flutter checklist:**
- [ ] Add `soma_<component>.dart` in `flutter/lib/src/components/<category>/`
- [ ] Re-export it from that category's barrel file (`<category>.dart`)
- [ ] Add a gallery screen: `flutter/example/lib/screens/<component>_screen.dart` + register in `flutter/example/lib/main.dart` `_navEntries`
- [ ] Read colors and spacing from `SomaTheme.of(context)` — no hardcoded values

**Token parity:**
- `web/playground/style/main.css` is the canonical token source. If your component needs a new token, add it there first, then mirror the value in `flutter/lib/src/theme/`.

---

## Verify Before Opening a PR

```sh
# Web
cd web && cargo check --target wasm32-unknown-unknown -p playground

# Flutter
flutter analyze
```

Both must pass cleanly with no warnings.

---

## PR Expectations

- One component or one focused fix per PR — no bundled refactors.
- No speculative abstractions or "we might need this later" code.
- Match existing style: naming, indentation, token usage, module layout.
- Update both web and Flutter if the change affects a shared component or token.

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to abide by its terms.
