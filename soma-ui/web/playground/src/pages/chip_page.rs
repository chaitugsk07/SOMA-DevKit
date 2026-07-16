use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Chip, ChipVariant};

fn parse_chip_variant(s: &str) -> ChipVariant {
    match s {
        "Secondary" => ChipVariant::Secondary,
        "Outline" => ChipVariant::Outline,
        _ => ChipVariant::Default,
    }
}

#[component]
pub fn ChipPage() -> impl IntoView {
    let chips = RwSignal::new(vec!["Design", "Engineering", "Product"]);
    let variant = RwSignal::new(ChipVariant::Default);
    let removable = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Chip".to_string())
            subtitle=Signal::derive(move || "Pill-shaped tag with optional remove action.".to_string())
        >
            <PreviewPanel>
                <div class="flex flex-wrap gap-2">
                    {move || chips.get().into_iter().map(|label| {
                        let label_owned = label.to_string();
                        let label_for_remove = label_owned.clone();
                        let cur_variant = variant.get();
                        let cur_removable = removable.get();
                        view! {
                            <Chip
                                variant=cur_variant
                                removable=cur_removable
                                on_remove=Callback::new(move |_| {
                                    chips.update(|v| v.retain(|s| *s != label_for_remove));
                                })
                            >
                                {label_owned}
                            </Chip>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Variant">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| variant.set(parse_chip_variant(&event_target_value(&e)))
                    >
                        <option value="Default">"Default"</option>
                        <option value="Secondary">"Secondary"</option>
                        <option value="Outline">"Outline"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Removable">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| removable.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="flex flex-wrap gap-3">
                    <Chip variant=ChipVariant::Default>"Default"</Chip>
                    <Chip variant=ChipVariant::Secondary>"Secondary"</Chip>
                    <Chip variant=ChipVariant::Outline>"Outline"</Chip>
                </div>
            </div>
        </PageShell>
    }
}
