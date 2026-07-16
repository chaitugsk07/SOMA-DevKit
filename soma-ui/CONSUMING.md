# Using soma-ui in Another Repo

## Cargo dependency

**Active development (sibling clone):**
```toml
[dependencies]
soma-ui = { path = "../soma-ui/web/packages/ui" }
```

**Stable pin:**
```toml
soma-ui = { git = "https://github.com/soma-platform/soma-ui", package = "soma-ui", tag = "v0.1.0" }
```

**Track a branch:**
```toml
soma-ui = { git = "https://github.com/soma-platform/soma-ui", package = "soma-ui", branch = "main" }
```

> **Note:** The git-dependency form above is Rust-only. With a git dep, `theme/` lands under
> `~/.cargo/git/checkouts/<hash>/` at an unstable path, so the `require()`/`@import`/`copy-dir`
> theme wiring described below will break. The Tailwind/CSS theme layer requires the
> **path-dependency form** until a prebuilt `soma-ui.css` ships (deferred).

## Tailwind setup

soma-ui ships a Tailwind **preset** at `web/theme/tailwind.preset.js` that contains all tokens (colors, shadows, fonts, keyframes). Consumer's `tailwind.config.js`:

```js
module.exports = {
  presets: [require('../soma-ui/web/theme/tailwind.preset.js')],
  content: [
    "./src/**/*.rs",
    // soma-ui source MUST be in content so component classes aren't purged:
    "../soma-ui/web/packages/ui/**/*.rs",
  ],
  plugins: [],
};
```

> `content` is NOT in the preset — globs are path-relative per consumer. Keep your own.

## CSS tokens

Consumer's main CSS file:

```css
/* assuming your entry CSS is at style/main.css and soma-ui is a sibling dir */
@import "../../soma-ui/web/theme/tokens.css";

@tailwind base;
@tailwind components;
@tailwind utilities;
```

`theme/tokens.css` contains the `:root`/`.dark` CSS-variable blocks and `@font-face` rules for Outfit and Rajdhani.

## Fonts

Fonts live at `web/theme/fonts/`. Consumer's `index.html` (Trunk):

```html
<!-- soma-ui is a sibling of this repo; index.html is at the repo root -->
<link data-trunk rel="copy-dir" href="../soma-ui/web/theme/fonts" data-target-path="fonts" />
```

Fonts will be served at `/fonts/...` — matching the `@font-face` src URLs in `tokens.css`.

## Git hook — keeping COMPONENTS.md in sync

A `hooks/pre-commit` script auto-regenerates `COMPONENTS.md` whenever component
source files are staged, so the catalog is never stale in a commit.

Fresh-clone setup (one command):

```sh
git config core.hooksPath hooks
```

`dev.sh` sets this automatically on first run (idempotent).

---

## CLAUDE.md block for the consumer repo

Paste this into the consumer repo's `CLAUDE.md`:

```
## UI components — soma-ui

All reusable UI components live ONLY in soma-ui (https://github.com/soma-platform/soma-ui).

- To discover available components: read `soma-ui/web/COMPONENTS.md` (generated catalog).
- Import any component as `use soma_ui::ComponentName;` — the public API is flat.
- NEVER create a UI component locally in this repo.
- If a component is missing or needs a new variant/prop: add it to soma-ui in the correct
  category folder (`web/packages/ui/src/components/<category>/`), re-run `cargo run -q -p catalog`
  (from `web/`) to regenerate `web/COMPONENTS.md`, then use the updated component here.
```
