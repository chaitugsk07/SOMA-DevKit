use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Spinner, SpinnerSize};

fn parse_spinner_size(s: &str) -> SpinnerSize {
    match s {
        "Sm" => SpinnerSize::Sm,
        "Lg" => SpinnerSize::Lg,
        _ => SpinnerSize::Md,
    }
}

#[component]
pub fn SpinnerPage() -> impl IntoView {
    let size = RwSignal::new(SpinnerSize::Md);

    view! {
        <PageShell
            title=Signal::derive(move || "Spinner".to_string())
            subtitle=Signal::derive(move || "Animated loading indicator.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Spinner size=size.get() />
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Size">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| size.set(parse_spinner_size(&event_target_value(&e)))
                    >
                        <option value="Sm">"Sm"</option>
                        <option value="Md" selected>"Md"</option>
                        <option value="Lg">"Lg"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            // All Sizes
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Sizes"</h2>
                <div class="flex flex-wrap items-center gap-6">
                    <div class="flex flex-col items-center gap-2">
                        <Spinner size=SpinnerSize::Sm />
                        <span class="text-xs text-muted-foreground">"Sm"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Spinner size=SpinnerSize::Md />
                        <span class="text-xs text-muted-foreground">"Md"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Spinner size=SpinnerSize::Lg />
                        <span class="text-xs text-muted-foreground">"Lg"</span>
                    </div>
                </div>
            </div>
        </PageShell>
    }
}
