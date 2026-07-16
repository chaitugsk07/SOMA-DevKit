use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Switch;

#[component]
pub fn SwitchPage() -> impl IntoView {
    let checked = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(move || "Switch".to_string())
            subtitle=Signal::derive(move || "A sliding toggle for boolean state.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <div class="flex items-center gap-3">
                        <Switch checked=checked disabled=disabled.get() />
                        <span class="text-sm text-foreground">
                            {move || if checked.get() { "On" } else { "Off" }}
                        </span>
                    </div>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Checked">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| checked.set(event_target_checked(&e))
                    />
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
