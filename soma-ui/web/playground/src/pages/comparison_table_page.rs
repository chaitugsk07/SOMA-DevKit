use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{ComparisonCell, ComparisonRow, ComparisonTable};

#[component]
pub fn ComparisonTablePage() -> impl IntoView {
    let headers = vec![
        "Free".to_string(),
        "Pro".to_string(),
        "Enterprise".to_string(),
    ];
    let rows = vec![
        ComparisonRow {
            label: "Migrations".to_string(),
            cells: vec![
                ComparisonCell::Text("Up to 10".to_string()),
                ComparisonCell::Yes,
                ComparisonCell::Yes,
            ],
        },
        ComparisonRow {
            label: "Advisory lock".to_string(),
            cells: vec![
                ComparisonCell::Yes,
                ComparisonCell::Yes,
                ComparisonCell::Yes,
            ],
        },
        ComparisonRow {
            label: "Checksum drift".to_string(),
            cells: vec![ComparisonCell::No, ComparisonCell::Yes, ComparisonCell::Yes],
        },
        ComparisonRow {
            label: "SSO".to_string(),
            cells: vec![ComparisonCell::No, ComparisonCell::No, ComparisonCell::Yes],
        },
    ];

    view! {
        <PageShell
            title=Signal::derive(|| "Comparison Table".to_string())
            subtitle=Signal::derive(|| "Feature comparison table with highlighted column support.".to_string())
        >
            <PreviewPanel>
                <ComparisonTable
                    headers=headers
                    rows=rows
                    highlight_col=1usize
                />
            </PreviewPanel>
        </PageShell>
    }
}
