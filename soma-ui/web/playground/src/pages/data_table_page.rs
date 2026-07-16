use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Column, DataTable};
use std::collections::HashMap;

fn sample_rows() -> Vec<HashMap<String, String>> {
    let data = [
        ("Alice Johnson", "alice@example.com", "Admin", "Active"),
        ("Bob Smith", "bob@example.com", "Editor", "Active"),
        ("Carol White", "carol@example.com", "Viewer", "Inactive"),
        ("David Brown", "david@example.com", "Editor", "Active"),
        ("Eve Davis", "eve@example.com", "Admin", "Active"),
        ("Frank Miller", "frank@example.com", "Viewer", "Pending"),
        ("Grace Wilson", "grace@example.com", "Editor", "Active"),
        ("Henry Moore", "henry@example.com", "Viewer", "Inactive"),
        ("Iris Taylor", "iris@example.com", "Admin", "Active"),
        ("Jack Anderson", "jack@example.com", "Editor", "Pending"),
        ("Karen Thomas", "karen@example.com", "Viewer", "Active"),
        ("Leo Jackson", "leo@example.com", "Editor", "Inactive"),
    ];
    data.iter()
        .map(|(name, email, role, status)| {
            let mut m = HashMap::new();
            m.insert("name".into(), name.to_string());
            m.insert("email".into(), email.to_string());
            m.insert("role".into(), role.to_string());
            m.insert("status".into(), status.to_string());
            m
        })
        .collect()
}

fn columns() -> Vec<Column> {
    vec![
        Column {
            key: "name".into(),
            header: "Name".into(),
            sortable: true,
            editable: false,
        },
        Column {
            key: "email".into(),
            header: "Email".into(),
            sortable: true,
            editable: false,
        },
        Column {
            key: "role".into(),
            header: "Role".into(),
            sortable: true,
            editable: false,
        },
        Column {
            key: "status".into(),
            header: "Status".into(),
            sortable: true,
            editable: false,
        },
    ]
}

fn parse_page_size(s: &str) -> usize {
    match s {
        "3" => 3,
        "5" => 5,
        "10" => 10,
        "0" => 0,
        _ => 5,
    }
}

#[component]
pub fn DataTablePage() -> impl IntoView {
    let selectable = RwSignal::new(true);
    let filterable = RwSignal::new(true);
    let page_size = RwSignal::new(5usize);

    view! {
        <div class="max-w-4xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">"Data Table"</h1>
                <p class="text-sm text-muted-foreground mt-1">"Sortable, filterable, paginated table with optional row selection."</p>
            </div>

            // Controls
            <ControlsPanel>
                <ControlRow label="Page size">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| page_size.set(parse_page_size(&event_target_value(&e)))
                    >
                        <option value="3">"3"</option>
                        <option value="5" selected>"5"</option>
                        <option value="10">"10"</option>
                        <option value="0">"All"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Row selection">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || selectable.get()
                        on:change=move |e| selectable.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Filter bar">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || filterable.get()
                        on:change=move |e| filterable.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // Preview
            <div class="bg-card border border-border rounded-md p-6 space-y-4">
                {move || view! {
                    <DataTable
                        columns=columns()
                        rows=sample_rows()
                        selectable=selectable.get()
                        filterable=filterable.get()
                        page_size=page_size.get()
                    />
                }}
                <p class="text-sm text-muted-foreground">
                    "Click a column header to sort. "
                    {move || format!("Page size: {}.", if page_size.get() == 0 { "All".to_string() } else { page_size.get().to_string() })}
                </p>
            </div>
        </div>
    }
}
