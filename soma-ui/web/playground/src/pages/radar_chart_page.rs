use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::{ChartPoint, ChartSeries, RadarChart, RadarVariant};

fn parse_radar_variant(s: &str) -> RadarVariant {
    match s {
        "Lines" => RadarVariant::Lines,
        "Dots" => RadarVariant::Dots,
        "Multiple" => RadarVariant::Multiple,
        _ => RadarVariant::Default,
    }
}

#[component]
pub fn RadarChartPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let variant = RwSignal::new(RadarVariant::Default);

    let sample = || {
        vec![
            ChartPoint {
                label: "Speed".into(),
                value: 80.0,
            },
            ChartPoint {
                label: "Power".into(),
                value: 65.0,
            },
            ChartPoint {
                label: "Range".into(),
                value: 90.0,
            },
            ChartPoint {
                label: "Agility".into(),
                value: 75.0,
            },
            ChartPoint {
                label: "Defense".into(),
                value: 55.0,
            },
            ChartPoint {
                label: "Stamina".into(),
                value: 85.0,
            },
        ]
    };

    let sample_series = || {
        vec![
            ChartSeries {
                points: vec![
                    ChartPoint {
                        label: "Speed".into(),
                        value: 80.0,
                    },
                    ChartPoint {
                        label: "Power".into(),
                        value: 65.0,
                    },
                    ChartPoint {
                        label: "Range".into(),
                        value: 90.0,
                    },
                    ChartPoint {
                        label: "Agility".into(),
                        value: 75.0,
                    },
                    ChartPoint {
                        label: "Defense".into(),
                        value: 55.0,
                    },
                    ChartPoint {
                        label: "Stamina".into(),
                        value: 85.0,
                    },
                ],
            },
            ChartSeries {
                points: vec![
                    ChartPoint {
                        label: "Speed".into(),
                        value: 60.0,
                    },
                    ChartPoint {
                        label: "Power".into(),
                        value: 85.0,
                    },
                    ChartPoint {
                        label: "Range".into(),
                        value: 70.0,
                    },
                    ChartPoint {
                        label: "Agility".into(),
                        value: 90.0,
                    },
                    ChartPoint {
                        label: "Defense".into(),
                        value: 75.0,
                    },
                    ChartPoint {
                        label: "Stamina".into(),
                        value: 50.0,
                    },
                ],
            },
            ChartSeries {
                points: vec![
                    ChartPoint {
                        label: "Speed".into(),
                        value: 70.0,
                    },
                    ChartPoint {
                        label: "Power".into(),
                        value: 50.0,
                    },
                    ChartPoint {
                        label: "Range".into(),
                        value: 80.0,
                    },
                    ChartPoint {
                        label: "Agility".into(),
                        value: 60.0,
                    },
                    ChartPoint {
                        label: "Defense".into(),
                        value: 90.0,
                    },
                    ChartPoint {
                        label: "Stamina".into(),
                        value: 65.0,
                    },
                ],
            },
        ]
    };

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().radar_chart_title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().radar_chart_subtitle}</p>
            </div>

            // Controls
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
                <div class="space-y-0">
                    <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3 border-b border-border last:border-0">
                        <span class="text-sm text-muted-foreground">"Variant"</span>
                        <select
                            class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                            on:change=move |e| variant.set(parse_radar_variant(&event_target_value(&e)))
                        >
                            <option value="Default">"Default"</option>
                            <option value="Lines">"Lines"</option>
                            <option value="Dots">"Dots"</option>
                            <option value="Multiple">"Multiple"</option>
                        </select>
                    </div>
                </div>
            </div>

            // Reactive preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex justify-center">
                <div class="w-64">
                    {move || view! {
                        <RadarChart variant=variant.get() data=sample() series=sample_series() />
                    }}
                </div>
            </div>

            // All variants gallery
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Default"</p>
                        <RadarChart variant=RadarVariant::Default data=sample() />
                    </div>
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Lines"</p>
                        <RadarChart variant=RadarVariant::Lines data=sample() />
                    </div>
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Dots"</p>
                        <RadarChart variant=RadarVariant::Dots data=sample() />
                    </div>
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3 font-medium">"Multiple"</p>
                        <RadarChart variant=RadarVariant::Multiple data=sample() series=sample_series() />
                    </div>
                </div>
            </div>
        </div>
    }
}
