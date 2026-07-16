use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, PageHeader};

#[component]
pub fn PageHeaderPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let show_subtitle = RwSignal::new(true);
    let show_action = RwSignal::new(true);
    let title_text = RwSignal::new("Secrets".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || s().nav_page_header.to_string())
            subtitle=Signal::derive(move || "Page title row with optional subtitle and actions slot.".to_string())
        >
            <PreviewPanel>
                {move || {
                    let subtitle = if show_subtitle.get() {
                        Some("Manage your environment secrets.".to_string())
                    } else {
                        None
                    };
                    view! {
                        <div class="w-full">
                            <PageHeader title=title_text.get() subtitle=subtitle>
                                <Show when=move || show_action.get()>
                                    <Button>"Add Secret"</Button>
                                </Show>
                            </PageHeader>
                        </div>
                    }
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Title">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || title_text.get()
                        on:input=move |e| title_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Show subtitle">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary"
                        prop:checked=move || show_subtitle.get()
                        on:change=move |e| show_subtitle.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Show action">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary"
                        prop:checked=move || show_action.get()
                        on:change=move |e| show_action.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
