# soma-ui

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org)
[![Flutter](https://img.shields.io/badge/Flutter-stable-02569B.svg)](https://flutter.dev)

A cross-platform design system for Leptos (web) and Flutter (mobile) — one component set, one design language, everywhere.

soma-ui ships the same component library twice: as Leptos/Rust components for web, and as Flutter widgets for mobile. The design language — Palantir-style slate/blue palette, Outfit body font, Rajdhani headings, full light/dark support — is identical across both targets. You paste the source into your project and own it; no black-box crates between you and your UI.

---

## Why soma-ui?

- **Copy-paste-first.** Inspired by shadcn: you take the source, not a dependency. The components are yours to read, modify, and ship.
- **One design language, two runtimes.** Web and Flutter components share the same token values, variants, and API shape. Switching targets doesn't mean re-learning the system.
- **Light/dark out of the box.** CSS custom properties on web; `SomaThemeProvider` on Flutter. No extra configuration.
- **No black boxes.** Leptos + Tailwind on web; pure Dart on Flutter. Nothing is hidden behind an opaque crate or package you can't inspect.

---

## Components

Both web and Flutter ship the same categories:

| Category       | Examples                                        |
|----------------|-------------------------------------------------|
| `inputs`       | Button, Input, Select, Checkbox, Toggle         |
| `data_display` | Badge, Card, Table, Stat, Avatar                |
| `layout`       | Stack, Grid, Divider, Container                 |
| `navigation`   | Sidebar, Tabs, Breadcrumb, NavLink              |
| `overlays`     | Modal, Drawer, Tooltip, Popover                 |
| `feedback`     | Alert, Toast, Spinner, Progress                 |
| `forms`        | Form, Field, Label, Validation                  |
| `disclosure`   | Accordion, Collapsible                          |
| `motion`       | Transition, Fade, Slide                         |
| `interaction`  | ThemeToggle, Ripple                             |
| `media`        | Image, Icon                                     |

---

## Quick Start

### Web (Leptos / Rust)

```toml
# Cargo.toml
[dependencies]
soma-ui = { path = "web/packages/ui" }
```

```rust
use soma_ui::Button;

view! {
    <Button variant="primary" on_click=move |_| log::info!("clicked")>
        "Get started"
    </Button>
}
```

Run the component playground (serves on `:9876`):

```sh
./dev.sh
```

### Flutter

```sh
flutter pub add soma_ui
```

```dart
import 'package:soma_ui/soma_ui.dart';

SomaButton(
  label: 'Get started',
  onPressed: () {},
)
```

Run the component gallery:

```sh
cd flutter/example && flutter run -d chrome
```

---

## Design System

**Tokens** are the single source of truth. On web, they live as CSS custom properties in `web/playground/style/main.css` — semantic names like `--color-surface`, `--color-primary`, `--radius-md`. Light and dark values are defined there and consumed by every Tailwind class in the components.

On Flutter, the same token values are ported into `flutter/lib/src/theme/` as `SomaColors`, `SomaTypography`, and `SomaSpacing`, surfaced through `SomaThemeProvider` (an `InheritedWidget`). If you change a token on web, update the Flutter theme to match — the web file is the canonical source.

**Fonts:** Outfit (body) and Rajdhani (headings) are self-hosted — bundled as TTF in `flutter/assets/fonts/` and loaded via Tailwind's font config on web. No Google Fonts CDN dependency.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions are welcome — new components, bug fixes, token improvements, or Flutter/web parity fixes.

---

## License

Apache 2.0 — see [LICENSE](LICENSE).
