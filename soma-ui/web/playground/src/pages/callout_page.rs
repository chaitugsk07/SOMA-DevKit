use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Callout, CalloutVariant};

fn parse_callout_variant(s: &str) -> CalloutVariant {
    match s {
        "Info" => CalloutVariant::Info,
        "Success" => CalloutVariant::Success,
        "Warning" => CalloutVariant::Warning,
        "Destructive" => CalloutVariant::Destructive,
        _ => CalloutVariant::Default,
    }
}

#[component]
pub fn CalloutPage() -> impl IntoView {
    let variant = RwSignal::new(CalloutVariant::Default);

    view! {
        <PageShell
            title=Signal::derive(|| "Callout".to_string())
            subtitle=Signal::derive(|| "Tinted left-bordered block for tips and notices.".to_string())
        >
            // Preview
            <PreviewPanel>
                {move || view! {
                    <Callout variant=variant.get() title="Note".to_string() class="max-w-sm w-full".to_string()>
                        <p class="text-sm text-muted-foreground">"This is a callout with an optional title."</p>
                    </Callout>
                }}
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Variant">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| variant.set(parse_callout_variant(&event_target_value(&e)))
                    >
                        <option value="Default">"Default"</option>
                        <option value="Info">"Info"</option>
                        <option value="Success">"Success"</option>
                        <option value="Warning">"Warning"</option>
                        <option value="Destructive">"Destructive"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="space-y-3">
                    <Callout variant=CalloutVariant::Default title="Default".to_string()>
                        <p class="text-sm text-muted-foreground">"Default callout content."</p>
                    </Callout>
                    <Callout variant=CalloutVariant::Info title="Info".to_string()>
                        <p class="text-sm text-muted-foreground">"Informational callout content."</p>
                    </Callout>
                    <Callout variant=CalloutVariant::Success title="Success".to_string()>
                        <p class="text-sm text-muted-foreground">"Successful callout content."</p>
                    </Callout>
                    <Callout variant=CalloutVariant::Warning title="Warning".to_string()>
                        <p class="text-sm text-muted-foreground">"Warning callout content."</p>
                    </Callout>
                    <Callout variant=CalloutVariant::Destructive title="Destructive".to_string()>
                        <p class="text-sm text-muted-foreground">"Destructive callout content."</p>
                    </Callout>
                </div>
            </div>
        </PageShell>
    }
}
