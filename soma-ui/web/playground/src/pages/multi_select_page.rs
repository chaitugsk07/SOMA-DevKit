use crate::ui::*;
use leptos::prelude::*;
use soma_ui::MultiSelect;

#[component]
pub fn MultiSelectPage() -> impl IntoView {
    let selected = RwSignal::new(Vec::<String>::new());
    let placeholder = RwSignal::new("Select languages…".to_string());

    let options = vec![
        ("rust".to_string(), "Rust".to_string()),
        ("typescript".to_string(), "TypeScript".to_string()),
        ("go".to_string(), "Go".to_string()),
        ("python".to_string(), "Python".to_string()),
        ("elixir".to_string(), "Elixir".to_string()),
        ("kotlin".to_string(), "Kotlin".to_string()),
    ];

    view! {
        <PageShell
            title=Signal::derive(move || "Multi Select".to_string())
            subtitle=Signal::derive(move || "Choose multiple options. Selected items appear as chips. Panel stays open.".to_string())
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-start gap-4 min-h-72">
                <div class="w-80">
                    {move || view! {
                        <MultiSelect selected=selected options=options.clone() placeholder=placeholder.get() />
                    }}
                </div>
                <p class="text-sm text-muted-foreground">
                    "Selected: "
                    <span class="text-foreground font-mono">
                        {move || {
                            let v = selected.get();
                            if v.is_empty() { "none".to_string() } else { v.join(", ") }
                        }}
                    </span>
                </p>
            </div>

            // Controls
            <ControlsPanel>
                <ControlRow label="Placeholder">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-52"
                        prop:value=move || placeholder.get()
                        on:input=move |e| placeholder.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Clear selection">
                    <button
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm hover:bg-accent"
                        on:click=move |_| selected.set(vec![])
                    >
                        "Clear"
                    </button>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
