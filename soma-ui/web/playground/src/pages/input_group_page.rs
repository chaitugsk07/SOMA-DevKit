use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Input, InputGroup, InputGroupAddon};

#[component]
pub fn InputGroupPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let addon_text = RwSignal::new("@".to_string());
    let placeholder = RwSignal::new("username".to_string());
    let addon_position = RwSignal::new("prefix".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Input Group".to_string())
            subtitle=Signal::derive(move || "Wraps an input with prefix or suffix addons in a unified border.".to_string())
        >
            // Preview
            <PreviewPanel>
                <div class="w-72">
                    {move || {
                        let pos = addon_position.get();
                        let addon = addon_text.get();
                        let ph = placeholder.get();
                        if pos == "suffix" {
                            view! {
                                <InputGroup>
                                    <Input
                                        value=value
                                        placeholder=ph
                                        class="border-0 focus:ring-0 rounded-none".to_string()
                                    />
                                    <InputGroupAddon>{addon}</InputGroupAddon>
                                </InputGroup>
                            }.into_any()
                        } else {
                            view! {
                                <InputGroup>
                                    <InputGroupAddon>{addon}</InputGroupAddon>
                                    <Input
                                        value=value
                                        placeholder=ph
                                        class="border-0 focus:ring-0 rounded-none".to_string()
                                    />
                                </InputGroup>
                            }.into_any()
                        }
                    }}
                </div>
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Addon text">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-32"
                        prop:value=move || addon_text.get()
                        on:input=move |e| addon_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Addon position">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| addon_position.set(event_target_value(&e))
                    >
                        <option value="prefix" selected>"Prefix"</option>
                        <option value="suffix">"Suffix"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Placeholder">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-40"
                        prop:value=move || placeholder.get()
                        on:input=move |e| placeholder.set(event_target_value(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
