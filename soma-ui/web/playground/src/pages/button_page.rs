use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{icons::icondata, Button, ButtonSize, ButtonVariant};

fn parse_variant(s: &str) -> ButtonVariant {
    match s {
        "Outline" => ButtonVariant::Outline,
        "Ghost" => ButtonVariant::Ghost,
        "Destructive" => ButtonVariant::Destructive,
        "Secondary" => ButtonVariant::Secondary,
        "Link" => ButtonVariant::Link,
        "Contrast" => ButtonVariant::Contrast,
        _ => ButtonVariant::Default,
    }
}

fn parse_size(s: &str) -> ButtonSize {
    match s {
        "Sm" => ButtonSize::Sm,
        "Lg" => ButtonSize::Lg,
        "Icon" => ButtonSize::Icon,
        _ => ButtonSize::Md,
    }
}

#[component]
pub fn ButtonPage() -> impl IntoView {
    let variant = RwSignal::new(ButtonVariant::Default);
    let size = RwSignal::new(ButtonSize::Md);
    let disabled = RwSignal::new(false);
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <PageShell
            title=Signal::derive(move || s().button_title.to_string())
            subtitle=Signal::derive(move || s().button_subtitle.to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Button variant=variant.get() size=size.get() disabled=disabled.get()>
                        "Click me"
                    </Button>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Variant">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| variant.set(parse_variant(&event_target_value(&e)))
                    >
                        <option value="Default">"Default"</option>
                        <option value="Outline">"Outline"</option>
                        <option value="Ghost">"Ghost"</option>
                        <option value="Destructive">"Destructive"</option>
                        <option value="Secondary">"Secondary"</option>
                        <option value="Link">"Link"</option>
                        <option value="Contrast">"Contrast"</option>
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
                        <option value="Icon">"Icon"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Disabled">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| disabled.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // All variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().all_variants}</h2>
                <div class="flex flex-wrap gap-3 mb-6">
                    <Button variant=ButtonVariant::Default>"Default"</Button>
                    <Button variant=ButtonVariant::Outline>"Outline"</Button>
                    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                    <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Link>"Link"</Button>
                    <Button variant=ButtonVariant::Contrast>"Contrast"</Button>
                </div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"Pill shape"</h2>
                <div class="flex flex-wrap gap-3 mb-6">
                    <Button pill=true>"Default pill"</Button>
                    <Button variant=ButtonVariant::Outline pill=true>"Outline pill"</Button>
                    <Button variant=ButtonVariant::Contrast pill=true>"Contrast pill"</Button>
                </div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"Icon slot"</h2>
                <div class="flex flex-wrap gap-3">
                    <Button leading_icon=icondata::LuPlus>"New item"</Button>
                    <Button trailing_icon=icondata::LuArrowRight>"Continue"</Button>
                    <Button size=ButtonSize::Icon aria_label="Search".to_string() leading_icon=icondata::LuSearch>{""}</Button>
                </div>
            </div>
        </PageShell>
    }
}
