use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, ButtonGroup, ButtonSize, ButtonVariant};

fn parse_variant(s: &str) -> ButtonVariant {
    match s {
        "Default" => ButtonVariant::Default,
        "Ghost" => ButtonVariant::Ghost,
        "Destructive" => ButtonVariant::Destructive,
        "Secondary" => ButtonVariant::Secondary,
        _ => ButtonVariant::Outline,
    }
}

fn parse_size(s: &str) -> ButtonSize {
    match s {
        "Sm" => ButtonSize::Sm,
        "Lg" => ButtonSize::Lg,
        _ => ButtonSize::Md,
    }
}

#[component]
pub fn ButtonGroupPage() -> impl IntoView {
    let variant = RwSignal::new(ButtonVariant::Outline);
    let size = RwSignal::new(ButtonSize::Md);

    view! {
        <PageShell
            title=Signal::derive(move || "Button Group".to_string())
            subtitle=Signal::derive(move || "A layout wrapper that connects adjacent buttons into a single control.".to_string())
        >
            // Preview
            <PreviewPanel>
                {move || view! {
                    <ButtonGroup>
                        <Button variant=variant.get() size=size.get()>"Left"</Button>
                        <Button variant=variant.get() size=size.get()>"Center"</Button>
                        <Button variant=variant.get() size=size.get()>"Right"</Button>
                    </ButtonGroup>
                }}
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Variant">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| variant.set(parse_variant(&event_target_value(&e)))
                    >
                        <option value="Outline" selected>"Outline"</option>
                        <option value="Default">"Default"</option>
                        <option value="Ghost">"Ghost"</option>
                        <option value="Destructive">"Destructive"</option>
                        <option value="Secondary">"Secondary"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Size">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| size.set(parse_size(&event_target_value(&e)))
                    >
                        <option value="Sm">"Sm"</option>
                        <option value="Md" selected>"Md"</option>
                        <option value="Lg">"Lg"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
