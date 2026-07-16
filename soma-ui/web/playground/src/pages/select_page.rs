use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Select, SelectContent, SelectItem};

#[component]
pub fn SelectPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let disabled_placeholder = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(move || "Select".to_string())
            subtitle=Signal::derive(move || "Single-choice dropdown. Closes on selection or outside click.".to_string())
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-start gap-4 min-h-72">
                <div class="w-64">
                    {move || {
                        let placeholder = if disabled_placeholder.get() {
                            "No placeholder"
                        } else {
                            "Choose a fruit…"
                        };
                        view! {
                            <Select value=value placeholder=placeholder>
                                <SelectContent>
                                    <SelectItem value="apple">"Apple"</SelectItem>
                                    <SelectItem value="banana">"Banana"</SelectItem>
                                    <SelectItem value="cherry">"Cherry"</SelectItem>
                                    <SelectItem value="mango">"Mango"</SelectItem>
                                    <SelectItem value="orange">"Orange"</SelectItem>
                                </SelectContent>
                            </Select>
                        }
                    }}
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
                <ControlRow label="Hide placeholder">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| disabled_placeholder.set(event_target_checked(&e))
                    />
                </ControlRow>
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
