use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Progress;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let value = RwSignal::new(60.0f64);

    view! {
        <PageShell
            title=Signal::derive(|| "Progress".to_string())
            subtitle=Signal::derive(|| "Visual indicator of completion percentage.".to_string())
        >
            <PreviewPanel>
                <div class="w-full max-w-sm">
                    {move || view! { <Progress value=value.get() /> }}
                </div>
            </PreviewPanel>
            <ControlsPanel>
                <div class="flex flex-col gap-2">
                    <div class="flex items-center justify-between">
                        <span class="text-sm text-muted-foreground">"Value"</span>
                        <span class="text-sm text-foreground">{move || format!("{:.0}%", value.get())}</span>
                    </div>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        class="w-full"
                        prop:value=move || value.get().to_string()
                        on:input=move |e| {
                            if let Ok(v) = event_target_value(&e).parse::<f64>() {
                                value.set(v);
                            }
                        }
                    />
                </div>
            </ControlsPanel>
        </PageShell>
    }
}
