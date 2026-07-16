use crate::ui::*;
use leptos::prelude::*;
use soma_ui::DatePicker;

#[component]
pub fn DatePickerPage() -> impl IntoView {
    let value = RwSignal::new(None::<String>);
    let placeholder = RwSignal::new("Pick a date".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Date Picker".to_string())
            subtitle=Signal::derive(move || "Custom calendar popover. Stores ISO YYYY-MM-DD. Click outside or select a date to close.".to_string())
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-start gap-4 min-h-72">
                <div class="w-64">
                    {move || view! {
                        <DatePicker value=value placeholder=placeholder.get() />
                    }}
                </div>
                <p class="text-sm text-muted-foreground">
                    "Selected: "
                    <span class="text-foreground font-mono">
                        {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </p>
            </div>

            // Controls
            <ControlsPanel>
                <ControlRow label="Placeholder">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || placeholder.get()
                        on:input=move |e| placeholder.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Clear value">
                    <button
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm hover:bg-accent"
                        on:click=move |_| value.set(None)
                    >
                        "Clear"
                    </button>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
