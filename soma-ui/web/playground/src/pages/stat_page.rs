use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Stat, StatTrend};

#[component]
pub fn StatPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let show_delta = RwSignal::new(true);
    let value_text = RwSignal::new("12,480".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || s().nav_stat.to_string())
            subtitle=Signal::derive(move || "KPI metric card built on Card.".to_string())
        >
            <PreviewPanel>
                {move || {
                    if show_delta.get() {
                        view! {
                            <Stat
                                label="Total Secrets"
                                value=value_text.get()
                                delta=Some("12% vs last month".to_string())
                                trend=Some(StatTrend::Up)
                            />
                        }.into_any()
                    } else {
                        view! {
                            <Stat
                                label="Total Secrets"
                                value=value_text.get()
                            />
                        }.into_any()
                    }
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Value">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || value_text.get()
                        on:input=move |e| value_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Show delta">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary"
                        prop:checked=move || show_delta.get()
                        on:change=move |e| show_delta.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().all_variants}</h2>
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <Stat label="Secrets" value="1,024" delta=Some("8%".to_string()) trend=Some(StatTrend::Up) />
                    <Stat label="Errors" value="3" delta=Some("2".to_string()) trend=Some(StatTrend::Down) />
                    <Stat label="Latency" value="42ms" delta=Some("stable".to_string()) trend=Some(StatTrend::Neutral) />
                </div>
            </div>
        </PageShell>
    }
}
