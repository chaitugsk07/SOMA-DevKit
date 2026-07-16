use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Input;

#[component]
pub fn InputPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <PageShell
            title=Signal::derive(move || s().input_title.to_string())
            subtitle=Signal::derive(move || s().input_subtitle.to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <div class="w-full max-w-xs md:w-64">
                        <Input
                            value=value
                            placeholder="Type something...".to_string()
                            disabled=disabled.get()
                        />
                    </div>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Disabled">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary"
                        on:change=move |e| disabled.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Current value">
                    <span class="text-sm text-foreground font-mono">{move || {
                        let v = value.get();
                        if v.is_empty() { "(empty)".to_string() } else { v }
                    }}</span>
                </ControlRow>
            </ControlsPanel>

            // Examples
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().all_variants}</h2>
                <div class="space-y-4 max-w-sm">
                    {
                        let email = RwSignal::new(String::new());
                        let password = RwSignal::new(String::new());
                        view! {
                            <Input value=email input_type="email".to_string() placeholder="Email address".to_string() />
                            <Input value=password input_type="password".to_string() placeholder="Password".to_string() />
                        }
                    }
                </div>
            </div>
        </PageShell>
    }
}
