pub mod components;
pub use components::*;

pub mod charts;
pub use charts::*;

pub mod blocks;
pub use blocks::*;

pub mod screens;
pub use screens::{DashboardScreen, LoginScreen, MobileAppScreen, SettingsScreen};

/// Re-exports of the icon crates. Use `soma_ui::icons::Icon` to render an icon
/// and `soma_ui::icons::icondata` for icon constants (e.g. `icondata::LuCheck`).
pub mod icons {
    pub use icondata;
    pub use leptos_icons::Icon;
}

pub mod analytics;
pub use analytics::{AnalyticsPanel, Dashboard, PanelDef, SomaColumn, SomaResultSet};

pub mod hooks;
pub use hooks::*;

// ── Security-CNAPP convenience aliases ────────────────────────────────────────
/// `AppShell` is an alias for `ConsoleShell` — left-sidebar + topbar + scrollable main area.
pub use components::layout::console_shell::ConsoleShell as AppShell;
/// `SidebarNav` is an alias for `Sidebar` — icon+label nav with active-path highlighting.
pub use components::navigation::sidebar::Sidebar as SidebarNav;
pub use components::navigation::sidebar::SidebarGroup as SidebarNavGroup;
/// Re-export so callers can do `use soma_ui::SidebarNavItem` without reaching into submodules.
pub use components::navigation::sidebar::SidebarItem as SidebarNavItem;

/// Bundled stylesheet (design tokens + fonts + component utilities).
///
/// Inject once at your app root so every soma-ui component is styled:
///
/// ```rust,ignore
/// use soma_ui::STYLES;
/// view! { <style>{STYLES}</style> }
/// ```
///
/// Alternatively, write this string to a `.css` file and serve it via your
/// bundler (Trunk, Vite, etc.) instead of inlining it.
///
/// Fonts (Outfit, Rajdhani) are loaded from Google Fonts via `@import`.
/// If you need self-hosted fonts or custom tokens, use the Tailwind path-dep
/// workflow described in `CONSUMING.md` and regenerate with `build-css.sh`.
pub const STYLES: &str = include_str!("../style/soma-ui.css");
