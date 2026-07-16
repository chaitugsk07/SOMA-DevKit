use crate::ui::*;
use leptos::prelude::*;
use soma_ui::RadioButtonGroup;

#[component]
pub fn RadioButtonGroupPage() -> impl IntoView {
    let value = RwSignal::new("apple".to_string());
    let options = vec![
        ("apple".to_string(), "Apple".to_string()),
        ("banana".to_string(), "Banana".to_string()),
        ("cherry".to_string(), "Cherry".to_string()),
    ];

    view! {
        <PageShell
            title=Signal::derive(move || "Radio Button Group".to_string())
            subtitle=Signal::derive(move || "A set of mutually exclusive radio buttons bound to a shared value.".to_string())
        >
            // Preview
            <PreviewPanel>
                <div class="space-y-4">
                    <RadioButtonGroup
                        value=value
                        name="fruit".to_string()
                        options=options
                    />
                    <p class="text-xs text-muted-foreground">
                        "Selected: " {move || value.get()}
                    </p>
                </div>
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Value">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| value.set(event_target_value(&e))
                    >
                        <option value="apple" selected>"Apple"</option>
                        <option value="banana">"Banana"</option>
                        <option value="cherry">"Cherry"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
