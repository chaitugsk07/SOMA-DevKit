use crate::analytics::{analytics_panel::AnalyticsPanel, types::PanelDef};
use crate::components::Empty;
use leptos::prelude::*;

/// A responsive CSS-grid dashboard that renders a list of `PanelDef`s as
/// `AnalyticsPanel` tiles.
#[component]
pub fn Dashboard(panels: Vec<PanelDef>) -> impl IntoView {
    if panels.is_empty() {
        return view! {
            <Empty
                title="No panels".to_string()
                description="Add panels to your dashboard to see data here.".to_string()
            />
        }
        .into_any();
    }

    let tiles: Vec<_> = panels
        .into_iter()
        .map(|p| {
            view! {
                <AnalyticsPanel
                    title=p.title
                    chart_type=p.chart_type
                    result=p.result
                />
            }
        })
        .collect();

    view! {
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
            {tiles}
        </div>
    }
    .into_any()
}
