use leptos::prelude::*;
use soma_ui::{ChartPoint, StatTile};

fn rps_series() -> Vec<ChartPoint> {
    [
        980.0, 1050.0, 1120.0, 1090.0, 1200.0, 1180.0, 1250.0, 1300.0, 1280.0, 1350.0, 1320.0,
        1400.0, 1432.0,
    ]
    .iter()
    .enumerate()
    .map(|(i, &v)| ChartPoint {
        label: format!("t{i}"),
        value: v,
    })
    .collect()
}

fn error_series() -> Vec<ChartPoint> {
    [
        2.0, 3.0, 5.0, 4.0, 7.0, 6.0, 8.0, 5.0, 4.0, 3.0, 6.0, 9.0, 11.0,
    ]
    .iter()
    .enumerate()
    .map(|(i, &v)| ChartPoint {
        label: format!("t{i}"),
        value: v,
    })
    .collect()
}

fn latency_series() -> Vec<ChartPoint> {
    [
        12.0, 14.0, 11.0, 18.0, 22.0, 19.0, 15.0, 13.0, 16.0, 24.0, 21.0, 17.0, 14.0,
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
pub fn StatTilePage() -> impl IntoView {
    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">
                    "StatTile"
                </h1>
                <p class="text-sm text-muted-foreground mt-1">
                    "Single-value dashboard tile with optional delta and embedded Sparkline."
                </p>
            </div>

            // 3-up grid matching a typical observability dashboard
            <div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"With Sparkline"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <StatTile
                        label="Request rate".to_string()
                        value="1,432 /s".to_string()
                        delta=12.4_f64
                        spark=rps_series()
                    />
                    <StatTile
                        label="Error rate".to_string()
                        value="0.77%".to_string()
                        delta=-2.1_f64
                        spark=error_series()
                    />
                    <StatTile
                        label="p50 latency".to_string()
                        value="14 ms".to_string()
                        delta=-8.3_f64
                        spark=latency_series()
                    />
                </div>
            </div>

            // Minimal — no delta, no sparkline
            <div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"Minimal (no delta, no spark)"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <StatTile
                        label="Total requests".to_string()
                        value="2.4 M".to_string()
                    />
                    <StatTile
                        label="Uptime".to_string()
                        value="99.98%".to_string()
                    />
                    <StatTile
                        label="Active nodes".to_string()
                        value="12".to_string()
                    />
                </div>
            </div>

            // Delta only (no sparkline)
            <div>
                <h2 class="text-sm font-semibold text-foreground mb-4">"With delta only"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <StatTile
                        label="CPU utilisation".to_string()
                        value="64%".to_string()
                        delta=5.2_f64
                    />
                    <StatTile
                        label="Memory RSS".to_string()
                        value="1.8 GiB".to_string()
                        delta=-3.7_f64
                    />
                </div>
            </div>
        </div>
    }
}
