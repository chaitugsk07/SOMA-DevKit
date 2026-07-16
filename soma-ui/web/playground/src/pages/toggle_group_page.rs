use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn ToggleGroupPage() -> impl IntoView {
    let value = RwSignal::new("bold".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Toggle Group".to_string())
            subtitle=Signal::derive(move || "A set of connected toggle buttons with a single active selection.".to_string())
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-52">
                {move || view! {
                    <ToggleGroup value=value>
                        <ToggleGroupItem value="bold".to_string()>"Bold"</ToggleGroupItem>
                        <ToggleGroupItem value="italic".to_string()>"Italic"</ToggleGroupItem>
                        <ToggleGroupItem value="underline".to_string()>"Underline"</ToggleGroupItem>
                    </ToggleGroup>
                }}
                <p class="text-xs text-muted-foreground">
                    "Selected: " {move || value.get()}
                </p>
            </div>

            // Controls
            <ControlsPanel>
                <ControlRow label="Active item">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| value.set(event_target_value(&e))
                    >
                        <option value="bold" selected>"Bold"</option>
                        <option value="italic">"Italic"</option>
                        <option value="underline">"Underline"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
