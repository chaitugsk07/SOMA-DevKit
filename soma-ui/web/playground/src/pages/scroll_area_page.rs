use crate::ui::*;
use leptos::prelude::*;
use soma_ui::ScrollArea;

fn parse_height(s: &str) -> &'static str {
    match s {
        "32" => "max-h-32",
        "64" => "max-h-64",
        "96" => "max-h-96",
        _ => "max-h-48",
    }
}

#[component]
pub fn ScrollAreaPage() -> impl IntoView {
    let height_val = RwSignal::new("48".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Scroll Area".to_string())
            subtitle=Signal::derive(move || "A styled scrollable container with a custom scrollbar.".to_string())
        >
            <PreviewPanel>
                {move || {
                    let h_class = parse_height(&height_val.get());
                    let class = format!("{} w-64 border border-border rounded-md p-3", h_class);
                    view! {
                        <ScrollArea class=class>
                            <div class="space-y-2">
                                {(1u32..=20).map(|i| view! {
                                    <div class="text-sm text-muted-foreground">{format!("Item {i}")}</div>
                                }).collect_view()}
                            </div>
                        </ScrollArea>
                    }
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Max height">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| height_val.set(event_target_value(&e))
                    >
                        <option value="32">"Small (8rem)"</option>
                        <option value="48" selected>"Medium (12rem)"</option>
                        <option value="64">"Large (16rem)"</option>
                        <option value="96">"Extra large (24rem)"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
