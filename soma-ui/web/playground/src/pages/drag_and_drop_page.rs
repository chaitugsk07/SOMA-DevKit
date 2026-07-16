use crate::ui::*;
use leptos::prelude::*;
use soma_ui::DragAndDrop;

#[component]
pub fn DragAndDropPage() -> impl IntoView {
    let all_items: &'static [&'static str] = &[
        "Design system tokens",
        "Component library",
        "Playground explorer",
        "Documentation site",
        "CI / CD pipeline",
        "Accessibility audit",
        "Dark mode support",
    ];

    let items = RwSignal::new(vec![
        "Design system tokens".to_string(),
        "Component library".to_string(),
        "Playground explorer".to_string(),
        "Documentation site".to_string(),
        "CI / CD pipeline".to_string(),
    ]);

    let item_count = RwSignal::new(5usize);

    view! {
        <PageShell
            title=Signal::derive(move || "Drag and Drop".to_string())
            subtitle=Signal::derive(move || "Reorderable list via native HTML5 DnD. Drag any row to a new position.".to_string())
        >
            <PreviewPanel>
                <div class="w-full">
                    <DragAndDrop items=items />
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Number of items">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            let n: usize = event_target_value(&e).parse().unwrap_or(5);
                            item_count.set(n);
                            items.set(all_items.iter().take(n).map(|s| s.to_string()).collect());
                        }
                    >
                        <option value="3">"3"</option>
                        <option value="5" selected>"5"</option>
                        <option value="7">"7"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Reset order">
                    <button
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm hover:bg-accent transition-colors"
                        on:click=move |_| {
                            items.set(all_items.iter().take(item_count.get()).map(|s| s.to_string()).collect());
                        }
                    >
                        "Reset"
                    </button>
                </ControlRow>
            </ControlsPanel>

            <div class="bg-card border border-border rounded-md p-6">
                <p class="text-xs text-muted-foreground">"Current order:"</p>
                <p class="text-sm text-foreground mt-1">
                    {move || items.get().iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" → ")}
                </p>
            </div>
        </PageShell>
    }
}
