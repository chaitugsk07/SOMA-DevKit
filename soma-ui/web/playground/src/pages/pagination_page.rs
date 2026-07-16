use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Pagination;

fn parse_usize(s: &str, fallback: usize) -> usize {
    s.parse::<usize>().unwrap_or(fallback)
}

#[component]
pub fn PaginationPage() -> impl IntoView {
    let page = RwSignal::new(1usize);
    let total_pages = RwSignal::new(10usize);

    // Keep page in bounds when total changes
    let clamped_page = move || page.get().min(total_pages.get()).max(1);

    view! {
        <PageShell
            title=Signal::derive(move || "Pagination".to_string())
            subtitle=Signal::derive(move || "Navigate between pages with prev/next buttons and numbered page controls.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-52">
                {move || {
                    let total = total_pages.get();
                    // clamp page signal when total shrinks
                    if page.get() > total {
                        page.set(total);
                    }
                    view! {
                        <Pagination page=page total_pages=total />
                    }
                }}
                <p class="text-xs text-muted-foreground">
                    "Page " {clamped_page} " of " {move || total_pages.get()}
                </p>
            </div>

            <ControlsPanel>
                <ControlRow label="Total pages (1–20)">
                    <div class="flex items-center gap-3">
                        <input
                            type="range"
                            min="1"
                            max="20"
                            value="10"
                            class="w-32 accent-primary"
                            on:input=move |e| {
                                let n = parse_usize(&event_target_value(&e), 10);
                                total_pages.set(n);
                            }
                        />
                        <span class="text-sm text-foreground font-mono w-6">{move || total_pages.get()}</span>
                    </div>
                </ControlRow>
                <ControlRow label="Current page">
                    <span class="text-sm text-foreground font-mono">{clamped_page}</span>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
