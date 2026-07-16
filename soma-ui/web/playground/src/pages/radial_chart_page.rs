use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::{ChartPoint, ChartSeries, RadialChart, RadialVariant};

fn parse_radial_variant(s: &str) -> RadialVariant {
    match s {
        "Stacked" => RadialVariant::Stacked,
        "Labeled" => RadialVariant::Labeled,
        "Track" => RadialVariant::Track,
        _ => RadialVariant::Default,
    }
}

#[component]
pub fn RadialChartPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let variant = RwSignal::new(RadialVariant::Default);

    let sample = || {
        vec![
            ChartPoint {
                label: "CPU".into(),
                value: 72.0,
            },
            ChartPoint {
                label: "Memory".into(),
                value: 55.0,
            },
            ChartPoint {
                label: "Disk".into(),
                value: 88.0,
            },
            ChartPoint {
                label: "Network".into(),
                value: 40.0,
            },
        ]
    };

    let sample_series = || {
        vec![
            ChartSeries {
                points: vec![
                    ChartPoint {
                        label: "CPU".into(),
                        value: 72.0,
                    },
                    ChartPoint {
                        label: "Memory".into(),
                        value: 55.0,
                    },
                    ChartPoint {
                        label: "Disk".into(),
                        value: 88.0,
                    },
                    ChartPoint {
                        label: "Network".into(),
                        value: 40.0,
                    },
                ],
            },
            ChartSeries {
                points: vec![
                    ChartPoint {
                        label: "CPU".into(),
                        value: 15.0,
                    },
                    ChartPoint {
                        label: "Memory".into(),
                        value: 25.0,
                    },
                    ChartPoint {
                        label: "Disk".into(),
                        value: 8.0,
                    },
                    ChartPoint {
                        label: "Network".into(),
                        value: 30.0,
                    },
                ],
            },
            ChartSeries {
                points: vec![
                    ChartPoint {
                        label: "CPU".into(),
                        value: 13.0,
                    },
                    ChartPoint {
                        label: "Memory".into(),
                        value: 20.0,
                    },
                    ChartPoint {
                        label: "Disk".into(),
                        value: 4.0,
                    },
                    ChartPoint {
                        label: "Network".into(),
                        value: 30.0,
                    },
                ],
            },
        ]
    };

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().radial_chart_title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().radial_chart_subtitle}</p>
            </div>

            // Controls
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
                <div class="space-y-0">
                    <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3 border-b border-border last:border-0">
                        <span class="text-sm text-muted-foreground">"Variant"</span>
                        <select
                            class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                            on:change=move |e| variant.set(parse_radial_variant(&event_target_value(&e)))
                        >
                            <option value="Default">"Default"</option>
                            <option value="Track">"Track"</option>
                            <option value="Labeled">"Labeled"</option>
                            <option value="Stacked">"Stacked"</option>
                        </select>
                    </div>
                </div>
            </div>

            // Reactive preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex justify-center">
                <div class="w-64">
                    {move || view! {
                        <RadialChart variant=variant.get() data=sample() series=sample_series() />
                    }}
                </div>
            </div>

            // All variants gallery
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Default"</p>
                        <RadialChart variant=RadialVariant::Default data=sample() />
                    </div>
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Track"</p>
                        <RadialChart variant=RadialVariant::Track data=sample() />
                    </div>
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Labeled"</p>
                        <RadialChart variant=RadialVariant::Labeled data=sample() />
                    </div>
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Stacked"</p>
                        <RadialChart variant=RadialVariant::Stacked data=sample() series=sample_series() />
                    </div>
                </div>
            </div>
        </div>
    }
}
