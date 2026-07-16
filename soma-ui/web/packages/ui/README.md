# soma-ui

A [Leptos](https://leptos.dev) 0.8 CSR component library with 160+ accessible UI components, blocks, charts, and a ChartDB-style schema diagram.

Built on Tailwind CSS with a design-token theme (CSS custom properties), dark-mode support, and elevation shadows.

## Install

```toml
[dependencies]
soma-ui = "0.1"
leptos = { version = "^0.8", features = ["csr"] }
```

## Basic usage

```rust
use leptos::prelude::*;
use soma_ui::{Button, ButtonVariant, STYLES};

#[component]
fn App() -> impl IntoView {
    view! {
        // Inject styles once at the app root (see Styling section below)
        <style>{STYLES}</style>

        <Button variant=ButtonVariant::Default on_click=|_| log::info!("clicked")>
            "Click me"
        </Button>
    }
}
```

## Component catalog

See [COMPONENTS.md](https://github.com/chaitugsk07/soma-ui/blob/main/COMPONENTS.md) for the full list of available components, blocks, charts, and screens.

## Styling

soma-ui components are styled with Tailwind CSS utility classes and CSS custom property tokens. There are two ways to get the styles depending on how you consume the crate.

### (a) Registry consumers (crates.io)

Inject the bundled stylesheet via the `soma_ui::STYLES` constant. This string contains the compiled Tailwind utilities, all design tokens (`:root` and `.dark` blocks), and a Google Fonts `@import` for **Outfit** and **Rajdhani**.

```rust
use leptos::prelude::*;
use soma_ui::STYLES;

#[component]
fn Root() -> impl IntoView {
    view! {
        <style>{STYLES}</style>
        // ... rest of your app
    }
}
```

Alternatively, write the string to a `.css` file and serve it via Trunk or your bundler:

```rust
// In a build.rs or CLI tool:
std::fs::write("assets/soma-ui.css", soma_ui::STYLES).unwrap();
```

Then in `index.html`:
```html
<link rel="stylesheet" href="assets/soma-ui.css" />
```

Fonts (Outfit, Rajdhani) load from Google Fonts via the `@import` in the bundled CSS. If you need fully self-hosted fonts or a custom token set, use the Tailwind path-dep workflow below.

### (b) Tailwind users (path-dep / monorepo)

If you are consuming soma-ui as a path dependency within the soma-platform monorepo (or your own workspace), use the Tailwind preset at `web/theme/tailwind.preset.js` so your bundler picks up all utility classes at build time:

```js
// tailwind.config.js
const somaPreset = require('../soma-ui/web/theme/tailwind.preset.js');
module.exports = {
  presets: [somaPreset],
  content: ['./src/**/*.rs', '../soma-ui/web/packages/ui/src/**/*.rs'],
};
```

See [CONSUMING.md](https://github.com/chaitugsk07/soma-ui/blob/main/CONSUMING.md) for full setup instructions including self-hosted fonts and CSS variable customisation.

## Rebuilding the bundled CSS

When you add new Tailwind classes to components, regenerate `style/soma-ui.css` with:

```bash
bash packages/ui/build-css.sh
```

(Run from `soma-ui/web/`. Requires Node.js; pulls `tailwindcss@3` via npx.)

## License

Apache-2.0 — see [LICENSE](LICENSE).
