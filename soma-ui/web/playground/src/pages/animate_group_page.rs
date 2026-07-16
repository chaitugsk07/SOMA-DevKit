use crate::ui::*;
use leptos::prelude::*;
use soma_ui::AnimateGroup;

#[component]
pub fn AnimateGroupPage() -> impl IntoView {
    let key = RwSignal::new(0u32);
    view! {
        <PageShell
            title=Signal::derive(|| "Animate Group".to_string())
            subtitle=Signal::derive(|| "Applies a staggered entrance animation to direct children.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center min-h-52 gap-6">
                {move || {
                    let _k = key.get();
                    view! {
                        <AnimateGroup class="flex flex-col gap-3 w-64".to_string()>
                            <div class="bg-muted rounded-md p-3 text-sm text-foreground">"First item"</div>
                            <div class="bg-muted rounded-md p-3 text-sm text-foreground">"Second item"</div>
                            <div class="bg-muted rounded-md p-3 text-sm text-foreground">"Third item"</div>
                        </AnimateGroup>
                    }
                }}
                <button
                    class="text-xs text-muted-foreground hover:text-foreground underline"
                    on:click=move |_| key.update(|k| *k += 1)
                >
                    "Replay"
                </button>
            </div>
        </PageShell>
    }
}
