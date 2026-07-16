use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::{BarChart, BarVariant, ChartPoint, ChartSeries};

fn sample() -> Vec<ChartPoint> {
    vec![
        ChartPoint {
            label: "Q1".into(),
            value: 120.0,
        },
        ChartPoint {
            label: "Q2".into(),
            value: 85.0,
        },
        ChartPoint {
            label: "Q3".into(),
            value: 150.0,
        },
        ChartPoint {
            label: "Q4".into(),
            value: 110.0,
        },
    ]
}

fn sample_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Q1".into(),
                    value: 60.0,
                },
                ChartPoint {
                    label: "Q2".into(),
                    value: 45.0,
                },
                ChartPoint {
                    label: "Q3".into(),
                    value: 80.0,
                },
                ChartPoint {
                    label: "Q4".into(),
                    value: 55.0,
                },
            ],
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Q1".into(),
                    value: 40.0,
                },
                ChartPoint {
                    label: "Q2".into(),
                    value: 30.0,
                },
                ChartPoint {
                    label: "Q3".into(),
                    value: 50.0,
                },
                ChartPoint {
                    label: "Q4".into(),
                    value: 35.0,
                },
            ],
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Q1".into(),
                    value: 20.0,
                },
                ChartPoint {
                    label: "Q2".into(),
                    value: 10.0,
                },
                ChartPoint {
                    label: "Q3".into(),
                    value: 20.0,
                },
                ChartPoint {
                    label: "Q4".into(),
                    value: 20.0,
                },
            ],
        },
    ]
}

fn parse_bar_variant(s: &str) -> BarVariant {
    match s {
        "horizontal" => BarVariant::Horizontal,
        "stacked" => BarVariant::Stacked,
        "grouped" => BarVariant::Grouped,
        _ => BarVariant::Default,
    }
}

#[component]
pub fn BarChartPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let variant = RwSignal::new(BarVariant::Default);

    let all_variants = [
        ("default", "Default (Vertical)"),
        ("horizontal", "Horizontal"),
        ("stacked", "Stacked"),
        ("grouped", "Grouped"),
    ];

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">
                    {move || s().bar_chart_title}
                </h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().bar_chart_subtitle}</p>
            </div>

            // Controls panel
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
                <div class="flex items-center gap-3">
                    <label class="text-sm text-muted-foreground" for="bar-variant-select">
                        "Variant"
                    </label>
                    <select
                        id="bar-variant-select"
                        class="rounded border border-border bg-background text-foreground text-sm px-2 py-1 focus:outline-none focus:ring-1 focus:ring-primary"
                        on:change=move |e| {
                            let val = event_target_value(&e);
                            variant.set(parse_bar_variant(&val));
                        }
                    >
                        {all_variants.iter().map(|(value, label)| {
                            view! { <option value=*value>{*label}</option> }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
            </div>

            // Live preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12">
                {move || {
                    let v = variant.get();
                    let needs_series = matches!(v, BarVariant::Stacked | BarVariant::Grouped);
                    if needs_series {
                        view! {
                            <BarChart variant=v data=sample() series=sample_series() />
                        }.into_any()
                    } else {
                        view! {
                            <BarChart variant=v data=sample() />
                        }.into_any()
                    }
                }}
            </div>

            // All variants gallery
            <div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Default (Vertical)"</p>
                        <BarChart variant=BarVariant::Default data=sample() />
                    </div>
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Horizontal"</p>
                        <BarChart variant=BarVariant::Horizontal data=sample() />
                    </div>
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Stacked"</p>
                        <BarChart variant=BarVariant::Stacked data=sample() series=sample_series() />
                    </div>
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Grouped"</p>
                        <BarChart variant=BarVariant::Grouped data=sample() series=sample_series() />
                    </div>
                </div>
            </div>
        </div>
    }
}
