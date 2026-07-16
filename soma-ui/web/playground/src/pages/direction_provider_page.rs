use crate::ui::*;
use leptos::prelude::*;
use soma_ui::DirectionProvider;

#[component]
pub fn DirectionProviderPage() -> impl IntoView {
    let dir = RwSignal::new("ltr".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Direction Provider".to_string())
            subtitle=Signal::derive(move || "Wraps a subtree in a div with a 'dir' attribute to set LTR or RTL text direction.".to_string())
        >
            <PreviewPanel>
                {move || {
                    let d = dir.get();
                    let sample = if d == "rtl" {
                        "من اليمين إلى اليسار: هذا النص يتدفق من اليمين إلى اليسار."
                    } else {
                        "Left to right: This text flows from left to right as expected."
                    };
                    view! {
                        <DirectionProvider dir=d>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground w-full max-w-md">
                                {sample}
                            </div>
                        </DirectionProvider>
                    }
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Direction">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| dir.set(event_target_value(&e))
                    >
                        <option value="ltr" selected>"LTR (left to right)"</option>
                        <option value="rtl">"RTL (right to left)"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            <div class="bg-card border border-border rounded-md p-6 space-y-6">
                <h2 class="text-sm font-semibold text-foreground mb-2">"All Variants"</h2>
                <div class="space-y-2">
                    <p class="text-xs text-muted-foreground uppercase tracking-widest font-semibold">"LTR (default)"</p>
                    <DirectionProvider dir="ltr">
                        <div class="rounded-md border border-border p-4 text-sm text-foreground">
                            "Left to right: This text flows from left to right as expected."
                        </div>
                    </DirectionProvider>
                </div>
                <div class="space-y-2">
                    <p class="text-xs text-muted-foreground uppercase tracking-widest font-semibold">"RTL"</p>
                    <DirectionProvider dir="rtl">
                        <div class="rounded-md border border-border p-4 text-sm text-foreground">
                            "من اليمين إلى اليسار: هذا النص يتدفق من اليمين إلى اليسار."
                        </div>
                    </DirectionProvider>
                </div>
            </div>
        </PageShell>
    }
}
