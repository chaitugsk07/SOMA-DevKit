use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Column, DataGrid};
use std::collections::HashMap;

fn sample_rows() -> Vec<HashMap<String, String>> {
    let data = [
        ("Wireless Mouse", "29.99", "142"),
        ("USB-C Hub", "49.99", "87"),
        ("Mechanical Keyboard", "89.99", "34"),
        ("Monitor Stand", "39.99", "210"),
        ("Webcam HD", "79.99", "56"),
        ("Desk Lamp", "24.99", "315"),
        ("Cable Organizer", "14.99", "489"),
        ("Laptop Stand", "54.99", "73"),
    ];
    data.iter()
        .map(|(product, price, stock)| {
            let mut m = HashMap::new();
            m.insert("product".into(), product.to_string());
            m.insert("price".into(), price.to_string());
            m.insert("stock".into(), stock.to_string());
            m
        })
        .collect()
}

#[component]
pub fn DataGridPage() -> impl IntoView {
    let rows = RwSignal::new(sample_rows());
    let height_class = RwSignal::new("max-h-96".to_string());

    let columns = vec![
        Column {
            key: "product".into(),
            header: "Product".into(),
            sortable: false,
            editable: false,
        },
        Column {
            key: "price".into(),
            header: "Price ($)".into(),
            sortable: false,
            editable: true,
        },
        Column {
            key: "stock".into(),
            header: "Stock".into(),
            sortable: false,
            editable: true,
        },
    ];

    view! {
        <PageShell
            title=Signal::derive(|| "Data Grid".to_string())
            subtitle=Signal::derive(|| "Inline-editable grid with sticky header and pinned first column.".to_string())
        >
            // Controls
            <ControlsPanel>
                <ControlRow label="Max height">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| height_class.set(event_target_value(&e))
                    >
                        <option value="max-h-48">"Small (192px)"</option>
                        <option value="max-h-72">"Medium (288px)"</option>
                        <option value="max-h-96" selected>"Large (384px)"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Reset data">
                    <button
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm hover:bg-muted transition-colors"
                        on:click=move |_| rows.set(sample_rows())
                    >
                        "Reset"
                    </button>
                </ControlRow>
            </ControlsPanel>

            // Preview
            <div class="bg-card border border-border rounded-md p-6 space-y-4">
                {move || view! {
                    <DataGrid
                        columns=columns.clone()
                        rows=rows
                        height_class=height_class.get()
                    />
                }}
                <p class="text-sm text-muted-foreground">
                    "Double-click a Price or Stock cell to edit. Press Enter or click away to commit. Escape cancels."
                </p>
                <p class="text-sm text-muted-foreground">
                    {move || format!("{} rows", rows.get().len())}
                </p>
            </div>
        </PageShell>
    }
}
