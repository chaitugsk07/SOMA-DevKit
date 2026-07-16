use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::{ChartPoint, PieChart, PieVariant};

#[component]
pub fn PieChartPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let data = vec![
        ChartPoint {
            label: "Alpha".into(),
            value: 40.0,
        },
        ChartPoint {
            label: "Beta".into(),
            value: 25.0,
        },
        ChartPoint {
            label: "Gamma".into(),
            value: 20.0,
        },
        ChartPoint {
            label: "Delta".into(),
            value: 15.0,
        },
    ];

    let variant = RwSignal::new(PieVariant::Pie);

    let parse_variant = move |ev: leptos::web_sys::Event| {
        use leptos::wasm_bindgen::JsCast;
        let val = ev
            .target()
            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlSelectElement>().ok())
            .map(|s| s.value())
            .unwrap_or_default();
        if let Ok(v) = val.parse::<PieVariant>() {
            variant.set(v);
        }
    };

    let all_variants = [
        ("Pie", PieVariant::Pie),
        ("Donut", PieVariant::Donut),
        ("Labeled", PieVariant::Labeled),
        ("Half (Gauge)", PieVariant::Half),
    ];

    let gallery: Vec<_> = all_variants
        .iter()
        .map(|(label, v)| {
            let v = v.clone();
            let d = data.clone();
            view! {
                <div class="bg-card border border-border rounded-md p-4 space-y-2">
                    <p class="text-xs font-medium text-muted-foreground">{*label}</p>
                    <div class="flex justify-center">
                        <div class="w-40">
                            <PieChart data=d variant=v />
                        </div>
                    </div>
                </div>
            }
        })
        .collect();

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().pie_chart_title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().pie_chart_subtitle}</p>
            </div>

            // Controls
            <div class="bg-card border border-border rounded-md p-6 space-y-4">
                <h2 class="text-sm font-semibold text-foreground">{"Controls"}</h2>
                <div class="flex items-center gap-3">
                    <label class="text-sm text-muted-foreground" for="pie-variant-select">{"Variant"}</label>
                    <select
                        id="pie-variant-select"
                        class="bg-background border border-border rounded px-2 py-1 text-sm text-foreground"
                        on:change=parse_variant
                    >
                        <option value="Pie">{"Pie"}</option>
                        <option value="Donut">{"Donut"}</option>
                        <option value="Labeled">{"Labeled"}</option>
                        <option value="Half">{"Half (Gauge)"}</option>
                    </select>
                </div>
            </div>

            // Reactive preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex justify-center">
                <div class="w-48">
                    {move || {
                        let v = variant.get();
                        let d = data.clone();
                        view! { <PieChart data=d variant=v /> }
                    }}
                </div>
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
