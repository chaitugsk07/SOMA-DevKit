use leptos::prelude::*;
use soma_ui::{ChartPoint, ChartSeries, Heatmap};

/// Latency-bucket × time heatmap sample.
/// Rows (series) = latency buckets, top-to-bottom: "0–5ms" … ">500ms".
/// Columns (points) = time slots T0…T11.
fn latency_heatmap() -> (Vec<ChartSeries>, Vec<String>) {
    // (bucket_label, [count per time slot])
    let buckets: &[(&str, &[f64])] = &[
        (
            "0–5ms",
            &[
                120.0, 130.0, 125.0, 140.0, 135.0, 150.0, 145.0, 160.0, 155.0, 170.0, 165.0, 180.0,
            ],
        ),
        (
            "5–10ms",
            &[
                80.0, 85.0, 90.0, 88.0, 95.0, 100.0, 98.0, 105.0, 102.0, 110.0, 108.0, 115.0,
            ],
        ),
        (
            "10–25ms",
            &[
                40.0, 42.0, 45.0, 48.0, 50.0, 55.0, 52.0, 60.0, 58.0, 65.0, 62.0, 70.0,
            ],
        ),
        (
            "25–50ms",
            &[
                15.0, 18.0, 20.0, 22.0, 25.0, 28.0, 30.0, 35.0, 32.0, 38.0, 36.0, 40.0,
            ],
        ),
        (
            "50–100ms",
            &[
                5.0, 6.0, 8.0, 7.0, 10.0, 12.0, 11.0, 15.0, 13.0, 18.0, 16.0, 20.0,
            ],
        ),
        (
            "100–500ms",
            &[2.0, 2.0, 3.0, 3.0, 4.0, 5.0, 4.0, 6.0, 5.0, 8.0, 7.0, 10.0],
        ),
        (
            ">500ms",
            &[0.0, 1.0, 0.0, 1.0, 1.0, 2.0, 1.0, 3.0, 2.0, 4.0, 3.0, 5.0],
        ),
    ];

    let time_labels: Vec<String> = (0..12).map(|i| format!("T{i:02}")).collect();

    let mut series = Vec::new();
    let mut row_labels = Vec::new();

    for (bucket, counts) in buckets {
        let points = counts
            .iter()
            .zip(time_labels.iter())
            .map(|(&v, label)| ChartPoint {
                label: label.clone(),
                value: v,
            })
            .collect();
        series.push(ChartSeries { points });
        row_labels.push(bucket.to_string());
    }

    (series, row_labels)
}

#[component]
pub fn HeatmapPage() -> impl IntoView {
    let (series, row_labels) = latency_heatmap();

    // Smaller demo: 3 buckets × 6 time slots
    let small_series = vec![
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "00:00".into(),
                    value: 200.0,
                },
                ChartPoint {
                    label: "04:00".into(),
                    value: 180.0,
                },
                ChartPoint {
                    label: "08:00".into(),
                    value: 350.0,
                },
                ChartPoint {
                    label: "12:00".into(),
                    value: 420.0,
                },
                ChartPoint {
                    label: "16:00".into(),
                    value: 390.0,
                },
                ChartPoint {
                    label: "20:00".into(),
                    value: 280.0,
                },
            ],
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "00:00".into(),
                    value: 50.0,
                },
                ChartPoint {
                    label: "04:00".into(),
                    value: 40.0,
                },
                ChartPoint {
                    label: "08:00".into(),
                    value: 120.0,
                },
                ChartPoint {
                    label: "12:00".into(),
                    value: 160.0,
                },
                ChartPoint {
                    label: "16:00".into(),
                    value: 140.0,
                },
                ChartPoint {
                    label: "20:00".into(),
                    value: 80.0,
                },
            ],
        },
        ChartSeries {
            points: vec![
                ChartPoint {
                    label: "00:00".into(),
                    value: 5.0,
                },
                ChartPoint {
                    label: "04:00".into(),
                    value: 3.0,
                },
                ChartPoint {
                    label: "08:00".into(),
                    value: 15.0,
                },
                ChartPoint {
                    label: "12:00".into(),
                    value: 25.0,
                },
                ChartPoint {
                    label: "16:00".into(),
                    value: 20.0,
                },
                ChartPoint {
                    label: "20:00".into(),
                    value: 10.0,
                },
            ],
        },
    ];
    let small_labels = vec![
        "<10ms".to_string(),
        "10–100ms".to_string(),
        ">100ms".to_string(),
    ];

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">
                    "Heatmap"
                </h1>
                <p class="text-sm text-muted-foreground mt-1">
                    "2D grid — Y axis: latency buckets (rows), X axis: time (columns), shade: request count."
                </p>
            </div>

            // Full latency heatmap
            <div class="bg-card border border-border rounded-md p-6 space-y-2">
                <h2 class="text-sm font-semibold text-foreground">"Latency bucket × time (7 rows × 12 columns)"</h2>
                <p class="text-xs text-muted-foreground">
                    "Darker = more requests in that latency bucket at that time slot."
                </p>
                <Heatmap series=series row_labels=row_labels />
            </div>

            // Smaller demo
            <div class="bg-card border border-border rounded-md p-6 space-y-2">
                <h2 class="text-sm font-semibold text-foreground">"Request volume by hour (3 buckets × 6 hours)"</h2>
                <Heatmap series=small_series row_labels=small_labels />
            </div>

            // Edge case: empty
            <div class="bg-card border border-border rounded-md p-6 space-y-2">
                <h2 class="text-sm font-semibold text-foreground">"Empty (no data)"</h2>
                <Heatmap />
            </div>
        </div>
    }
}
