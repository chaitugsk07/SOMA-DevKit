use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Marquee;

#[component]
pub fn MarqueePage() -> impl IntoView {
    let tags: &'static [&'static str] = &[
        "Leptos",
        "Rust",
        "WebAssembly",
        "Tailwind",
        "Soma UI",
        "Dark Theme",
        "CSR",
    ];
    let reverse = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(|| "Marquee".to_string())
            subtitle=Signal::derive(|| "Continuously scrolling content track — children are duplicated for a seamless loop.".to_string())
        >
            // Preview
            <PreviewPanel>
                <div class="w-full">
                    {move || view! {
                        <Marquee reverse=reverse.get() class="w-full".to_string()>
                            {tags.iter().map(|t| view! {
                                <span class="mx-3 bg-muted text-muted-foreground text-sm px-3 py-1 rounded-full whitespace-nowrap">{*t}</span>
                            }).collect_view()}
                        </Marquee>
                    }}
                </div>
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Reverse direction">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| reverse.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="space-y-4">
                    <Marquee class="w-full".to_string()>
                        {tags.iter().map(|t| view! {
                            <span class="mx-3 bg-muted text-muted-foreground text-sm px-3 py-1 rounded-full whitespace-nowrap">{*t}</span>
                        }).collect_view()}
                    </Marquee>
                    <Marquee reverse=true class="w-full".to_string()>
                        {tags.iter().map(|t| view! {
                            <span class="mx-3 bg-secondary text-secondary-foreground text-sm px-3 py-1 rounded-full whitespace-nowrap">{*t}</span>
                        }).collect_view()}
                    </Marquee>
                </div>
            </div>
        </PageShell>
    }
}
