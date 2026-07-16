use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Badge, BadgeVariant};

fn parse_variant(s: &str) -> BadgeVariant {
    match s {
        "Secondary" => BadgeVariant::Secondary,
        "Destructive" => BadgeVariant::Destructive,
        "Outline" => BadgeVariant::Outline,
        "Success" => BadgeVariant::Success,
        _ => BadgeVariant::Default,
    }
}

#[component]
pub fn BadgePage() -> impl IntoView {
    let variant = RwSignal::new(BadgeVariant::Default);
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <PageShell
            title=Signal::derive(move || s().badge_title.to_string())
            subtitle=Signal::derive(move || s().badge_subtitle.to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Badge variant=variant.get()>"Badge"</Badge>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Variant">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| variant.set(parse_variant(&event_target_value(&e)))
                    >
                        <option value="Default">"Default"</option>
                        <option value="Secondary">"Secondary"</option>
                        <option value="Destructive">"Destructive"</option>
                        <option value="Outline">"Outline"</option>
                        <option value="Success">"Success"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            // All variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().all_variants}</h2>
                <div class="flex flex-wrap gap-3">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                    <Badge variant=BadgeVariant::Success>"Success"</Badge>
                </div>
            </div>
        </PageShell>
    }
}
