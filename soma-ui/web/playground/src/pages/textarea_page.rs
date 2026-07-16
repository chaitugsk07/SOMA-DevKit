use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Textarea;

#[component]
pub fn TextareaPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let rows = RwSignal::new(4i32);
    let disabled = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(move || "Textarea".to_string())
            subtitle=Signal::derive(move || "A multi-line text input field with controlled value.".to_string())
        >
            // Preview
            <PreviewPanel>
                <div class="w-80 space-y-2">
                    {move || view! {
                        <Textarea
                            value=value
                            placeholder="Type something…".to_string()
                            rows=rows.get()
                            disabled=disabled.get()
                        />
                    }}
                    <p class="text-xs text-muted-foreground">
                        {move || format!("{} characters", value.get().len())}
                    </p>
                </div>
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Rows">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            if let Ok(v) = event_target_value(&e).parse::<i32>() {
                                rows.set(v);
                            }
                        }
                    >
                        <option value="2">"2"</option>
                        <option value="4" selected>"4"</option>
                        <option value="6">"6"</option>
                        <option value="8">"8"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Disabled">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| disabled.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
