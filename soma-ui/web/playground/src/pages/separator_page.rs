use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Orientation, Separator};

fn parse_orientation(s: &str) -> Orientation {
    match s {
        "Vertical" => Orientation::Vertical,
        _ => Orientation::Horizontal,
    }
}

#[component]
pub fn SeparatorPage() -> impl IntoView {
    let orientation = RwSignal::new(Orientation::Horizontal);
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <PageShell
            title=Signal::derive(move || s().separator_title)
            subtitle=Signal::derive(move || s().separator_subtitle)
        >
            // Preview
            <PreviewPanel>
                {move || match orientation.get() {
                    Orientation::Horizontal => view! {
                        <div class="flex flex-col gap-3 w-48">
                            <span class="text-sm text-muted-foreground">"Above the line"</span>
                            <Separator orientation=Orientation::Horizontal />
                            <span class="text-sm text-muted-foreground">"Below the line"</span>
                        </div>
                    }.into_any(),
                    Orientation::Vertical => view! {
                        <div class="flex items-center gap-4 h-12">
                            <span class="text-sm text-muted-foreground">"Section A"</span>
                            <Separator orientation=Orientation::Vertical />
                            <span class="text-sm text-muted-foreground">"Section B"</span>
                        </div>
                    }.into_any(),
                }}
            </PreviewPanel>

            // Controls
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().controls}</h2>
                <div class="space-y-0">
                    <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3">
                        <span class="text-sm text-muted-foreground">{move || s().label_orientation}</span>
                        <select
                            class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                            on:change=move |e| orientation.set(parse_orientation(&event_target_value(&e)))
                        >
                            <option value="Horizontal" selected>"Horizontal"</option>
                            <option value="Vertical">"Vertical"</option>
                        </select>
                    </div>
                </div>
            </div>

            // All variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().all_variants}</h2>
                <div class="flex flex-col gap-6">
                    <div class="flex flex-col gap-2">
                        <span class="text-xs text-muted-foreground font-semibold uppercase tracking-widest">"Horizontal"</span>
                        <div class="flex flex-col gap-3 w-full max-w-xs">
                            <span class="text-sm text-muted-foreground">"Above the line"</span>
                            <Separator orientation=Orientation::Horizontal />
                            <span class="text-sm text-muted-foreground">"Below the line"</span>
                        </div>
                    </div>
                    <div class="flex flex-col gap-2">
                        <span class="text-xs text-muted-foreground font-semibold uppercase tracking-widest">"Vertical"</span>
                        <div class="flex items-center gap-4 h-12">
                            <span class="text-sm text-muted-foreground">"Section A"</span>
                            <Separator orientation=Orientation::Vertical />
                            <span class="text-sm text-muted-foreground">"Section B"</span>
                        </div>
                    </div>
                </div>
            </div>
        </PageShell>
    }
}
