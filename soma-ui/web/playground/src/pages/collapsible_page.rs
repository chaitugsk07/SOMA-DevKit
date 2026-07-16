use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, Collapsible};

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(|| "Collapsible".to_string())
            subtitle=Signal::derive(|| "A controlled show/hide container. The caller owns the open signal.".to_string())
        >
            <PreviewPanel>
                <div class="flex flex-col items-center justify-center gap-4 w-full">
                    <Button on:click=move |_| open.update(|v| *v = !*v)>
                        {move || if open.get() { "Hide content" } else { "Show content" }}
                    </Button>
                    <div class="w-full max-w-sm">
                        <Collapsible open=open class="w-full".to_string()>
                            <div class="rounded-md border border-border bg-muted/50 p-4 text-sm text-foreground">
                                "This content is collapsible. It animates in from the top when shown."
                            </div>
                        </Collapsible>
                    </div>
                </div>
            </PreviewPanel>

            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
                <div class="flex items-center justify-between py-3">
                    <span class="text-sm text-muted-foreground">"Open"</span>
                    <span class="text-sm text-foreground font-mono">{move || if open.get() { "true" } else { "false" }}</span>
                </div>
            </div>
        </PageShell>
    }
}
