use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::{AreaChart, AreaVariant, ChartPoint, ChartSeries};

fn sample() -> Vec<ChartPoint> {
    vec![
        ChartPoint {
            label: "Jan".into(),
            value: 40.0,
        },
        ChartPoint {
            label: "Feb".into(),
            value: 65.0,
        },
        ChartPoint {
            label: "Mar".into(),
            value: 55.0,
        },
        ChartPoint {
            label: "Apr".into(),
            value: 80.0,
        },
        ChartPoint {
            label: "May".into(),
            value: 72.0,
        },
        ChartPoint {
            label: "Jun".into(),
            value: 95.0,
        },
    ]
}

fn sample_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Jan".into(),
                    value: 40.0,
                },
                ChartPoint {
                    label: "Feb".into(),
                    value: 65.0,
                },
                ChartPoint {
                    label: "Mar".into(),
                    value: 55.0,
                },
                ChartPoint {
                    label: "Apr".into(),
                    value: 80.0,
                },
                ChartPoint {
                    label: "May".into(),
                    value: 72.0,
                },
                ChartPoint {
                    label: "Jun".into(),
                    value: 95.0,
                },
            ],
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Jan".into(),
                    value: 25.0,
                },
                ChartPoint {
                    label: "Feb".into(),
                    value: 30.0,
                },
                ChartPoint {
                    label: "Mar".into(),
                    value: 28.0,
                },
                ChartPoint {
                    label: "Apr".into(),
                    value: 35.0,
                },
                ChartPoint {
                    label: "May".into(),
                    value: 40.0,
                },
                ChartPoint {
                    label: "Jun".into(),
                    value: 38.0,
                },
            ],
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Jan".into(),
                    value: 15.0,
                },
                ChartPoint {
                    label: "Feb".into(),
                    value: 20.0,
                },
                ChartPoint {
                    label: "Mar".into(),
                    value: 18.0,
                },
                ChartPoint {
                    label: "Apr".into(),
                    value: 25.0,
                },
                ChartPoint {
                    label: "May".into(),
                    value: 22.0,
                },
                ChartPoint {
                    label: "Jun".into(),
                    value: 30.0,
                },
            ],
        },
    ]
}

fn parse_area_variant(s: &str) -> AreaVariant {
    match s {
        "linear" => AreaVariant::Linear,
        "step" => AreaVariant::Step,
        "gradient" => AreaVariant::Gradient,
        "stacked" => AreaVariant::Stacked,
        _ => AreaVariant::Default,
    }
}

#[component]
pub fn AreaChartPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let variant = RwSignal::new(AreaVariant::Default);

    let all_variants = [
        ("default", "Default (Smooth)"),
        ("linear", "Linear"),
        ("step", "Step"),
        ("gradient", "Gradient"),
        ("stacked", "Stacked"),
    ];

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">
                    {move || s().area_chart_title}
                </h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().area_chart_subtitle}</p>
            </div>

            // Controls panel
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
                <div class="flex items-center gap-3">
                    <label class="text-sm text-muted-foreground" for="area-variant-select">
                        "Variant"
                    </label>
                    <select
                        id="area-variant-select"
                        class="rounded border border-border bg-background text-foreground text-sm px-2 py-1 focus:outline-none focus:ring-1 focus:ring-primary"
                        on:change=move |e| {
                            let val = event_target_value(&e);
                            variant.set(parse_area_variant(&val));
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
                    let needs_series = matches!(v, AreaVariant::Stacked);
                    if needs_series {
                        view! {
                            <AreaChart variant=v data=sample() series=sample_series() />
                        }.into_any()
                    } else {
                        view! {
                            <AreaChart variant=v data=sample() />
                        }.into_any()
                    }
                }}
            </div>

            // All variants gallery
            <div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Default (Smooth)"</p>
                        <AreaChart variant=AreaVariant::Default data=sample() />
                    </div>
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Linear"</p>
                        <AreaChart variant=AreaVariant::Linear data=sample() />
                    </div>
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Step"</p>
                        <AreaChart variant=AreaVariant::Step data=sample() />
                    </div>
                    <div class="bg-card border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-2">"Gradient"</p>
                        <AreaChart variant=AreaVariant::Gradient data=sample() />
                    </div>
                    <div class="bg-card border border-border rounded-md p-4 md:col-span-2">
                        <p class="text-xs text-muted-foreground mb-2">"Stacked"</p>
                        <AreaChart variant=AreaVariant::Stacked data=sample() series=sample_series() />
                    </div>
                </div>
            </div>
        </div>
    }
}
