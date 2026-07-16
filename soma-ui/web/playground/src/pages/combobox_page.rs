use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Combobox;

#[component]
pub fn ComboboxPage() -> impl IntoView {
    let value = RwSignal::new(String::new());

    let options = vec![
        ("react".to_string(), "React".to_string()),
        ("leptos".to_string(), "Leptos".to_string()),
        ("svelte".to_string(), "Svelte".to_string()),
        ("vue".to_string(), "Vue".to_string()),
        ("angular".to_string(), "Angular".to_string()),
        ("solid".to_string(), "SolidJS".to_string()),
        ("htmx".to_string(), "HTMX".to_string()),
        ("dioxus".to_string(), "Dioxus".to_string()),
    ];

    view! {
        <PageShell
            title=Signal::derive(move || "Combobox".to_string())
            subtitle=Signal::derive(move || "Select with type-to-filter search. Options filtered case-insensitively.".to_string())
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-start gap-4 min-h-72">
                <div class="w-64">
                    <Combobox value=value options=options placeholder="Search frameworks…" />
                </div>
                <p class="text-sm text-muted-foreground">
                    "Selected: "
                    <span class="text-foreground font-mono">
                        {move || {
                            let v = value.get();
                            if v.is_empty() { "none".to_string() } else { v }
                        }}
                    </span>
                </p>
            </div>

            // Controls
            <ControlsPanel>
                <ControlRow label="Clear selection">
                    <button
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm hover:bg-accent transition-colors"
                        on:click=move |_| value.set(String::new())
                    >
                        "Clear"
                    </button>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
