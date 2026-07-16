use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Empty;

#[component]
pub fn EmptyPage() -> impl IntoView {
    let title_text = RwSignal::new("No results found".to_string());
    let description_text = RwSignal::new("Try adjusting your search or filters.".to_string());
    let show_description = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Empty".to_string())
            subtitle=Signal::derive(move || "Empty state placeholder for lists and tables.".to_string())
        >
            <PreviewPanel>
                {move || {
                    if show_description.get() {
                        view! {
                            <Empty
                                title=title_text.get()
                                description=description_text.get()
                            />
                        }.into_any()
                    } else {
                        view! {
                            <Empty title=title_text.get() />
                        }.into_any()
                    }
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Title">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-64"
                        prop:value=move || title_text.get()
                        on:input=move |e| title_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Description">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-64"
                        prop:value=move || description_text.get()
                        on:input=move |e| description_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Show description">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_description.get()
                        on:change=move |e| show_description.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
