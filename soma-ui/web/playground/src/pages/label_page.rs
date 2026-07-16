use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Input, Label};

#[component]
pub fn LabelPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let label_text = RwSignal::new("Email address".to_string());
    let placeholder = RwSignal::new("you@example.com".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Label".to_string())
            subtitle=Signal::derive(move || "A form label linked to its control via the for attribute.".to_string())
        >
            <PreviewPanel>
                <div class="flex flex-col gap-2 w-64">
                    {move || view! {
                        <Label for_id="demo-input".to_string()>{label_text.get()}</Label>
                        <Input value=value placeholder=placeholder.get() />
                    }}
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Label text">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || label_text.get()
                        on:input=move |e| label_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Placeholder">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || placeholder.get()
                        on:input=move |e| placeholder.set(event_target_value(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
