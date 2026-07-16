use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Pressable;

fn parse_label(s: &str) -> &'static str {
    match s {
        "Save" => "Save",
        "Cancel" => "Cancel",
        "Submit" => "Submit",
        "Delete" => "Delete",
        _ => "Press me",
    }
}

#[component]
pub fn PressablePage() -> impl IntoView {
    let label = RwSignal::new("Press me");

    view! {
        <PageShell
            title=Signal::derive(|| "Pressable".to_string())
            subtitle=Signal::derive(|| "A wrapper that shrinks on press — gives physical feedback to any clickable element.".to_string())
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex items-center justify-center min-h-52 gap-6 flex-wrap">
                {move || view! {
                    <Pressable>
                        <div class="bg-primary text-primary-foreground px-5 py-2.5 rounded-md text-sm font-medium cursor-pointer select-none">
                            {label.get()}
                        </div>
                    </Pressable>
                }}
            </div>

            // Controls
            <ControlsPanel>
                <ControlRow label="Button label">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| label.set(parse_label(&event_target_value(&e)))
                    >
                        <option value="Press me">"Press me"</option>
                        <option value="Save">"Save"</option>
                        <option value="Cancel">"Cancel"</option>
                        <option value="Submit">"Submit"</option>
                        <option value="Delete">"Delete"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="flex flex-wrap gap-4">
                    <Pressable>
                        <div class="bg-primary text-primary-foreground px-5 py-2.5 rounded-md text-sm font-medium cursor-pointer select-none">
                            "Press me"
                        </div>
                    </Pressable>
                    <Pressable>
                        <div class="bg-secondary text-secondary-foreground px-5 py-2.5 rounded-md text-sm font-medium cursor-pointer select-none">
                            "Secondary"
                        </div>
                    </Pressable>
                    <Pressable>
                        <div class="border border-border px-5 py-2.5 rounded-md text-sm text-foreground cursor-pointer select-none">
                            "Outline"
                        </div>
                    </Pressable>
                </div>
            </div>
        </PageShell>
    }
}
