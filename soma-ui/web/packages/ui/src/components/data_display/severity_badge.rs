use leptos::prelude::*;

/// Semantic severity level — shared across SeverityBadge, StatCard, SecurityGraph, DonutStat, IssueCard.
/// Derives `Copy` so it moves freely into closures without cloning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Severity {
    Critical,
    High,
    Medium,
    #[default]
    Low,
    Info,
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Severity::Critical => "Critical",
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Info => "Info",
        }
    }

    /// Inline CSS string for the badge pill (background + text).
    pub fn badge_style(self) -> &'static str {
        match self {
            Severity::Critical => "background-color:#e5484d;color:#fff",
            Severity::High => "background-color:#f5820a;color:#fff",
            Severity::Medium => "background-color:#f5c518;color:#1a1a1a",
            Severity::Low => "background-color:#3fb950;color:#fff",
            Severity::Info => "background-color:#3b82f6;color:#fff",
        }
    }

    /// Hex accent color — for rings, accent bars, and SVG fills.
    pub fn hex(self) -> &'static str {
        match self {
            Severity::Critical => "#e5484d",
            Severity::High => "#f5820a",
            Severity::Medium => "#f5c518",
            Severity::Low => "#3fb950",
            Severity::Info => "#3b82f6",
        }
    }
}

/// Severity pill badge with the canonical color for Critical/High/Medium/Low/Info.
#[component]
pub fn SeverityBadge(
    severity: Severity,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!(
        "inline-flex items-center whitespace-nowrap rounded border border-transparent px-2.5 py-0.5 text-xs font-semibold {}",
        class
    );
    view! {
        <span class=combined style=severity.badge_style()>{severity.label()}</span>
    }
}
