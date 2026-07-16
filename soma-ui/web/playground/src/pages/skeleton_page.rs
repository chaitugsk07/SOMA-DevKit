use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Skeleton;

#[component]
pub fn SkeletonPage() -> impl IntoView {
    let count = RwSignal::new(3usize);

    view! {
        <PageShell
            title=Signal::derive(|| "Skeleton".to_string())
            subtitle=Signal::derive(|| "Placeholder loading state for content.".to_string())
        >
            // Preview
            <PreviewPanel>
                <div class="space-y-3 w-full max-w-sm">
                    {move || (0..count.get()).map(|_| view! {
                        <Skeleton class="h-4 w-full".to_string() />
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
                        value="3"
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
