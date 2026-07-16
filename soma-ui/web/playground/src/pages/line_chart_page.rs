use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::{ChartPoint, ChartSeries, LineChart, LineVariant};

#[component]
pub fn LineChartPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let sample = vec![
        ChartPoint {
            label: "Mon".into(),
            value: 30.0,
        },
        ChartPoint {
            label: "Tue".into(),
            value: 55.0,
        },
        ChartPoint {
            label: "Wed".into(),
            value: 45.0,
        },
        ChartPoint {
            label: "Thu".into(),
            value: 70.0,
        },
        ChartPoint {
            label: "Fri".into(),
            value: 60.0,
        },
        ChartPoint {
            label: "Sat".into(),
            value: 85.0,
        },
        ChartPoint {
            label: "Sun".into(),
            value: 75.0,
        },
    ];

    let sample_series = vec![
        ChartSeries {
            points: sample.clone(),
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Mon".into(),
                    value: 20.0,
                },
                ChartPoint {
                    label: "Tue".into(),
                    value: 35.0,
                },
                ChartPoint {
                    label: "Wed".into(),
                    value: 50.0,
                },
                ChartPoint {
                    label: "Thu".into(),
                    value: 40.0,
                },
                ChartPoint {
                    label: "Fri".into(),
                    value: 55.0,
                },
                ChartPoint {
                    label: "Sat".into(),
                    value: 45.0,
                },
                ChartPoint {
                    label: "Sun".into(),
                    value: 65.0,
                },
            ],
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "Mon".into(),
                    value: 10.0,
                },
                ChartPoint {
                    label: "Tue".into(),
                    value: 20.0,
                },
                ChartPoint {
                    label: "Wed".into(),
                    value: 15.0,
                },
                ChartPoint {
                    label: "Thu".into(),
                    value: 30.0,
                },
                ChartPoint {
                    label: "Fri".into(),
                    value: 25.0,
                },
                ChartPoint {
                    label: "Sat".into(),
                    value: 40.0,
                },
                ChartPoint {
                    label: "Sun".into(),
                    value: 35.0,
                },
            ],
        },
    ];

    let variant = RwSignal::new(LineVariant::Default);

    let parse_variant = move |ev: leptos::web_sys::Event| {
        use leptos::wasm_bindgen::JsCast;
        let val = ev
            .target()
            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlSelectElement>().ok())
            .map(|s| s.value())
            .unwrap_or_default();
        if let Ok(v) = val.parse::<LineVariant>() {
            variant.set(v);
        }
    };

    let all_variants = [
        ("Default", LineVariant::Default),
        ("Linear", LineVariant::Linear),
        ("Step", LineVariant::Step),
        ("Dots", LineVariant::Dots),
        ("Multiple", LineVariant::Multiple),
    ];

    let gallery: Vec<_> = all_variants
        .iter()
        .map(|(label, v)| {
            let v = v.clone();
            let d = sample.clone();
            let ser = sample_series.clone();
            view! {
                <div class="bg-card border border-border rounded-md p-4 space-y-2">
                    <p class="text-xs font-medium text-muted-foreground">{*label}</p>
                    <LineChart data=d series=ser variant=v />
                </div>
            }
        })
        .collect();

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().line_chart_title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().line_chart_subtitle}</p>
            </div>

            // Controls
            <div class="bg-card border border-border rounded-md p-6 space-y-4">
                <h2 class="text-sm font-semibold text-foreground">{"Controls"}</h2>
                <div class="flex items-center gap-3">
                    <label class="text-sm text-muted-foreground" for="line-variant-select">{"Variant"}</label>
                    <select
                        id="line-variant-select"
                        class="bg-background border border-border rounded px-2 py-1 text-sm text-foreground"
                        on:change=parse_variant
                    >
                        <option value="Default">{"Default (Smooth)"}</option>
                        <option value="Linear">{"Linear"}</option>
                        <option value="Step">{"Step"}</option>
                        <option value="Dots">{"Dots"}</option>
                        <option value="Multiple">{"Multiple Series"}</option>
                    </select>
                </div>
            </div>

            // Reactive preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12">
                {move || {
                    let v = variant.get();
                    let d = sample.clone();
                    let ser = sample_series.clone();
                    view! { <LineChart data=d series=ser variant=v /> }
                }}
            </div>

            // All variants gallery
            <div>
                <h2 class="text-sm font-semibold text-foreground mb-4">{"All Variants"}</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {gallery}
                </div>
            </div>
        </div>
    }
}
