use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Shimmer;

#[component]
pub fn ShimmerPage() -> impl IntoView {
    let count = RwSignal::new(4usize);

    view! {
        <PageShell
            title=Signal::derive(|| "Shimmer".to_string())
            subtitle=Signal::derive(|| "A shimmer skeleton placeholder for loading states.".to_string())
        >
            // Preview
            <PreviewPanel>
                <div class="w-72 space-y-3">
                    {move || (0..count.get()).map(|_| view! {
                        <Shimmer class="h-4 w-full".to_string() />
                    }).collect_view()}
                </div>
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3 border-b border-border last:border-0">
                    <span class="text-sm text-muted-foreground">"Count ("{move || count.get()}")"</span>
                    <input
                        type="range"
                        min="1"
                        max="8"
                        value="4"
                        class="w-32"
                        on:input=move |e| {
                            if let Ok(n) = event_target_value(&e).parse::<usize>() {
                                count.set(n);
                            }
                        }
                    />
                </div>
            </ControlsPanel>
        </PageShell>
    }
}
