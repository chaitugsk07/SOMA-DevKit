use leptos::prelude::*;

/// A single large-number metric tile. Numbers are in IBM Plex Mono.
/// For the live-metrics hero, wrap with an island that updates the value via SSE.
#[component]
pub fn MetricTile(
    /// Short label shown below the value, e.g. "RAM"
    #[prop(into)]
    label: String,
    /// The displayed value, e.g. "18"
    #[prop(into)]
    value: String,
    /// Unit shown in smaller text next to the value, e.g. "MB"
    #[prop(optional, into)]
    unit: String,
) -> impl IntoView {
    view! {
        <div class="metric-tile">
            <div class="metric-tile__number">
                <span class="metric-tile__value">{value}</span>
                {if unit.is_empty() { None } else {
                    Some(view! { <span class="metric-tile__unit">{unit}</span> })
                }}
            </div>
            <p class="metric-tile__label">{label}</p>
        </div>
    }
}

/// A horizontal row of MetricTile components.
#[component]
pub fn StatRow(children: Children) -> impl IntoView {
    view! {
        <div class="stat-row" role="list">
            {children()}
        </div>
    }
}
