//! soma-ui-site — SSR-safe Leptos component library for the SOMA public site.
//!
//! # Design tokens
//! [`TOKENS_CSS`] contains the CSS custom-property block (one source of truth).
//! The consuming site inlines it via `<style>{soma_ui_site::TOKENS_CSS}</style>`
//! so dashboards can adopt the same vocabulary later.
//!
//! # Render mode
//! This crate has `leptos` with `default-features = false`. The consuming binary
//! activates the render mode (`ssr`/`hydrate`/`csr`) and Cargo feature
//! unification propagates it here. For a standalone type-check, run:
//! `cargo check` (if leptos compiles without a mode) or add `--features ssr`
//! to the check command.

pub mod components;

pub use components::*;

/// Design token CSS custom properties — single source of truth for the SOMA
/// design system. Inline this in the page `<head>` as a `<style>` block so
/// all component CSS (served as a static file) can reference the variables.
pub const TOKENS_CSS: &str = include_str!("../assets/tokens.css");
