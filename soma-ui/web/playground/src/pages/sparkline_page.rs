use leptos::prelude::*;
use soma_ui::{ChartPoint, SparkVariant, Sparkline};

fn latency_series() -> Vec<ChartPoint> {
    // Simulated p50 latency (ms) over 20 time slots
    [
        12.0, 14.0, 11.0, 18.0, 22.0, 19.0, 15.0, 13.0, 16.0, 24.0, 28.0, 21.0, 17.0, 20.0, 14.0,
        12.0, 16.0, 23.0, 19.0, 15.0,
    ]
    .iter()
    .enumerate()
    .map(|(i, &v)| ChartPoint {
        label: format!("t{i}"),
        value: v,
    })
    .collect()
}

#[component]
pub fn SparklinePage() -> impl IntoView {
    let data = latency_series();

    let variants = [
        ("Line", SparkVariant::Line),
        ("Area", SparkVariant::Area),
        ("Bars", SparkVariant::Bars),
    ];

    let gallery: Vec<_> = variants
        .iter()
        .map(|(label, v)| {
            let v = v.clone();
            let d = data.clone();
            view! {
                <div class="bg-card border border-border rounded-md p-4 space-y-2">
                    <p class="text-xs font-medium text-muted-foreground">{*label}</p>
                    // Fixed-height container so the SVG has something to fill
                    <div class="h-8">
                        <Sparkline data=d variant=v />
                    </div>
                </div>
            }
        })
        .collect();

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">
                    "Sparkline"
                </h1>
                <p class="text-sm text-muted-foreground mt-1">
                    "Minimal axis-less trend lines for embedding in table rows and stat tiles."
                </p>
            </div>

            // Inline usage example
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"Inline usage (48 px height)"</h2>
                <div class="flex items-center gap-4">
                    <span class="text-sm text-muted-foreground">"p50 latency"</span>
                    <div class="w-32 h-8">
                        <Sparkline data=data.clone() />
                    </div>
                    <span class="text-sm font-semibold tabular-nums text-foreground">"15 ms"</span>
                </div>
            </div>

            // All variants gallery
            <div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"Variants"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    {gallery}
                </div>
            </div>
        </div>
    }
}
