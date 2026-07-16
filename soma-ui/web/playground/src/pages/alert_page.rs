use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Alert, AlertDescription, AlertTitle, AlertVariant};

fn parse_alert_variant(s: &str) -> AlertVariant {
    match s {
        "Destructive" => AlertVariant::Destructive,
        "Success" => AlertVariant::Success,
        "Warning" => AlertVariant::Warning,
        "Info" => AlertVariant::Info,
        _ => AlertVariant::Default,
    }
}

#[component]
pub fn AlertPage() -> impl IntoView {
    let variant = RwSignal::new(AlertVariant::Default);

    view! {
        <PageShell
            title=Signal::derive(move || "Alert".to_string())
            subtitle=Signal::derive(move || "Contextual feedback messages.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Alert variant=variant.get() class="max-w-sm".to_string()>
                        <AlertTitle>"Heads up!"</AlertTitle>
                        <AlertDescription>"This is a default alert message."</AlertDescription>
                    </Alert>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Variant">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| variant.set(parse_alert_variant(&event_target_value(&e)))
                    >
                        <option value="Default">"Default"</option>
                        <option value="Destructive">"Destructive"</option>
                        <option value="Success">"Success"</option>
                        <option value="Warning">"Warning"</option>
                        <option value="Info">"Info"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="space-y-3">
                    <Alert variant=AlertVariant::Default>
                        <AlertTitle>"Default"</AlertTitle>
                        <AlertDescription>"Default alert style."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Destructive>
                        <AlertTitle>"Destructive"</AlertTitle>
                        <AlertDescription>"Something went wrong."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Success>
                        <AlertTitle>"Success"</AlertTitle>
                        <AlertDescription>"Operation completed successfully."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Warning>
                        <AlertTitle>"Warning"</AlertTitle>
                        <AlertDescription>"Proceed with caution."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Info>
                        <AlertTitle>"Info"</AlertTitle>
                        <AlertDescription>"Here is some useful information."</AlertDescription>
                    </Alert>
                </div>
            </div>
        </PageShell>
    }
}
