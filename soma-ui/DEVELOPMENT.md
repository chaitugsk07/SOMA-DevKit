# Development & build hygiene

soma-ui is a cross-platform UI monorepo (web = Leptos/Trunk, mobile = Flutter).
Both stacks generate large, regenerable build caches. This file lists what is
disposable, how to reclaim it, and the settings that keep caches lean.

## Disposable cache directories

All of these are **gitignored** and safe to delete at any time — they rebuild on
the next `cargo`/`trunk`/`flutter` run. Nothing here is source.

| Path | Stack | Typical size |
|------|-------|--------------|
| `web/target/` | Rust/wasm build cache (workspace) | multi-GB |
| `web/dist/`, `web/playground/dist/` | Trunk output | tens of MB |
| `flutter/build/`, `flutter/example/build/` | Flutter build cache | tens–hundreds of MB |
| `flutter/**/.dart_tool/` | Dart package/tooling cache | tens of MB |
| `flutter/rust_bridge_reference/build/` | Flutter build cache | ~GB |
| `flutter/rust_bridge_reference/rust/target/` | Rust build cache | hundreds of MB |
| `flutter/example/android/.gradle/` | Gradle cache | varies |
| `.playwright-mcp/console-*.log`, `screenshots/`, `*.png` | QA artifacts | small |

Verify any path is ignored before deleting:

```sh
git check-ignore <path>   # prints the path if ignored, exits non-zero otherwise
```

## Reclaim space

```sh
# Rust caches (per crate / workspace — preferred; releases shared hardlinks too)
cd web && cargo clean
cd flutter/rust_bridge_reference/rust && cargo clean

# Everything else (gitignored build dirs + QA logs)
rm -rf web/dist web/playground/dist
rm -rf flutter/build flutter/example/build flutter/rust_bridge_reference/build
rm -rf flutter/.dart_tool flutter/example/.dart_tool flutter/rust_bridge_reference/.dart_tool
rm -f  .playwright-mcp/console-*.log

# Flutter's own cleaner (alternative to the rm -rf lines above)
cd flutter && flutter clean
cd flutter/example && flutter clean
```

Check space before/after with `df -h .`.

## Lean-build settings

- **Rust dev debuginfo** — `[profile.dev] debug = "line-tables-only"` is set in
  `web/Cargo.toml` (workspace root, applies to all members) and
  `flutter/rust_bridge_reference/rust/Cargo.toml`. Full debuginfo is what makes
  `target/` reach multiple GB; line tables keep panic line numbers without the
  bulk. Use `debug = false` to drop them entirely. **Profiles only take effect at
  the workspace root** — adding `[profile.*]` to a member crate is ignored by Cargo.
- **Ship release builds** — `trunk build --release` for web, `flutter build` (which
  defaults to release) for mobile. Keep `--release` / production builds for anything
  published; dev builds stay lean via the profile above.

## Why caches never get committed

The root `.gitignore` already covers every directory above (`/target`,
`**/target`, `/dist`, `**/dist`, `flutter/build/`, `**/.dart_tool/`,
`node_modules/`, `.DS_Store`, `.playwright-mcp/`, `*.png`, …). `git ls-files`
should always show **zero** build artifacts — if you add a new stack, add its
cache dirs to `.gitignore` first, then confirm with `git check-ignore`.
