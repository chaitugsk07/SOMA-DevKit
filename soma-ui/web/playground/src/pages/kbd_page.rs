use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Kbd;

#[component]
pub fn KbdPage() -> impl IntoView {
    let key_text = RwSignal::new("⌘".to_string());
    let combo_mode = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Kbd".to_string())
            subtitle=Signal::derive(move || "Keyboard key display.".to_string())
        >
            <PreviewPanel>
                {move || {
                    if combo_mode.get() {
                        view! {
                            <div class="flex items-center gap-1">
                                <Kbd>{key_text.get()}</Kbd>
                                <span class="text-muted-foreground text-sm">"+"</span>
                                <Kbd>"K"</Kbd>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <Kbd>{key_text.get()}</Kbd>
                        }.into_any()
                    }
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Key label">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-28"
                        prop:value=move || key_text.get()
                        on:input=move |e| key_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Show as combo">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || combo_mode.get()
                        on:change=move |e| combo_mode.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // Examples
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Examples"</h2>
                <div class="flex flex-wrap gap-3 items-center">
                    <Kbd>"Ctrl"</Kbd>
                    <Kbd>"Alt"</Kbd>
                    <Kbd>"Del"</Kbd>
                    <Kbd>"Tab"</Kbd>
                    <Kbd>"Esc"</Kbd>
                    <Kbd>"Enter"</Kbd>
                    <Kbd>"Space"</Kbd>
                </div>
            </div>
        </PageShell>
    }
}
