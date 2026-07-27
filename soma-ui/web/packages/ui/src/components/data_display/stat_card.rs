use crate::components::data_display::severity_badge::Severity;
use leptos::prelude::*;

/// Big-number metric card with an optional severity accent bar and delta indicator.
/// Matches the Wiz-style dashboard stat tile — severity color bleeds along the top edge.
#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    /// Optional severity that colors the top accent bar.
    #[prop(optional)]
    accent: Option<Severity>,
    /// Optional delta string (e.g. "+12%", "3 new").
    #[prop(optional)]
    delta: Option<String>,
    /// When true, delta renders green (↑); when false, red (↓).
    #[prop(default = false)]
    delta_positive: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let accent_bar = accent.map(|sev| {
        view! {
            <div class="h-[3px] w-full shrink-0" style=format!("background-color:{}", sev.hex()) />
        }
    });

    let delta_view = delta.map(|d| {
        let (chip, prefix) = if delta_positive {
            (
                "inline-flex items-center text-xs font-medium mt-2 px-2 py-0.5 rounded-full bg-success/10 text-success",
                "\u{2191} ",
            )
        } else {
            (
                "inline-flex items-center text-xs font-medium mt-2 px-2 py-0.5 rounded-full bg-destructive/10 text-destructive",
                "\u{2193} ",
            )
        };
        let text = format!("{}{}", prefix, d);
        view! { <span class=chip>{text}</span> }
    });

    let card = format!(
        "bg-card border border-border rounded-xl shadow-elev-sm overflow-hidden flex flex-col {}",
        class
    );

    view! {
        <div class=card>
            {accent_bar}
            <div class="p-3">
                <p class="text-xs font-medium uppercase tracking-[0.08em] text-muted-foreground">
                    {label}
                </p>
                <p class="text-2xl font-semibold tabular-nums text-foreground mt-1">
                    {value}
                </p>
                {delta_view}
            </div>
        </div>
    }
}
