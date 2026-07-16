use crate::ui::*;
use leptos::prelude::*;
use soma_ui::RadioButton;

#[component]
pub fn RadioButtonPage() -> impl IntoView {
    let checked = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Radio Button".to_string())
            subtitle=Signal::derive(move || "A single radio button with a styled indicator. Use RadioButtonGroup for grouped options.".to_string())
        >
            // Preview
            <PreviewPanel>
                {move || view! {
                    <RadioButton
                        value="option".to_string()
                        name="demo".to_string()
                        checked=checked
                    >
                        "Select this option"
                    </RadioButton>
                }}
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Checked">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| checked.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
