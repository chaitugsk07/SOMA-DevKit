use crate::charts::{ChartSeries, H, PAD_B, PAD_L, PAD_R, PAD_T, W};
use leptos::prelude::*;

/// 2D heatmap: rows = latency buckets, columns = time slots.
///
/// **Orientation:** `series[0]` is the FIRST (top) row; `series[last]` is the
/// bottom row. `row_labels[i]` aligns with `series[i]`.
///
/// Cell color is `hsl(var(--primary))` with `fill-opacity` normalised 0.05→1.0
/// across all cell values globally (0 → 0.05, max → 1.0).
///
/// Handles empty/ragged input gracefully — missing columns in a row are skipped.
#[component]
pub fn Heatmap(
    /// One `ChartSeries` per row (top-to-bottom). Each point's `value` is the
    /// cell intensity; `label` is used for x-axis tick labels (only the first
    /// series' labels are shown to avoid duplication).
    #[prop(default = vec![])]
    series: Vec<ChartSeries>,
    /// Y-axis bucket labels, one per series entry (aligned index-for-index).
    #[prop(default = vec![])]
    row_labels: Vec<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let svg_class = format!("w-full h-auto {}", class);

    if series.is_empty() {
        return view! {
            <svg viewBox="0 0 320 200" class=svg_class aria-hidden="true" />
        }
        .into_any();
    }

    let num_rows = series.len();
    // Number of columns = max points across all series
    let num_cols = series.iter().map(|s| s.points.len()).max().unwrap_or(0);

    if num_cols == 0 {
        return view! {
            <svg viewBox="0 0 320 200" class=svg_class aria-hidden="true" />
        }
        .into_any();
    }

    // Global min/max for opacity normalisation
    let global_max = series
        .iter()
        .flat_map(|s| s.points.iter().map(|p| p.value))
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let global_min = series
        .iter()
        .flat_map(|s| s.points.iter().map(|p| p.value))
        .fold(f64::INFINITY, f64::min);
    let val_range = (global_max - global_min).max(1.0);

    let plot_w = W - PAD_L - PAD_R;
    let plot_h = H - PAD_T - PAD_B;

    let cell_w = plot_w / num_cols as f64;
    let cell_h = plot_h / num_rows as f64;

    // Render cells
    let mut cells: Vec<AnyView> = Vec::new();

    for (row_idx, s) in series.iter().enumerate() {
        for (col_idx, p) in s.points.iter().enumerate() {
            let x = PAD_L + col_idx as f64 * cell_w;
            let y = PAD_T + row_idx as f64 * cell_h;
            // Map value → opacity [0.05, 1.0]
            let t = (p.value - global_min) / val_range;
            let opacity = 0.05 + t * 0.95;
            cells.push(
                view! {
                    <rect
                        x=format!("{x:.1}") y=format!("{y:.1}")
                        width=format!("{:.1}", cell_w - 1.0)
                        height=format!("{:.1}", cell_h - 1.0)
                        fill="hsl(var(--primary))"
                        fill-opacity=format!("{opacity:.3}")
                        rx="1"
                    />
                }
                .into_any(),
            );
        }
    }

    // Y-axis row labels (left side, one per row, vertically centered in cell)
    let y_labels: Vec<_> = row_labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let y = PAD_T + i as f64 * cell_h + cell_h / 2.0;
            let label = label.clone();
            view! {
                <text
                    x=format!("{:.1}", PAD_L - 3.0)
                    y=format!("{y:.1}")
                    text-anchor="end"
                    dominant-baseline="middle"
                    font-size="7"
                    class="fill-muted-foreground"
                >{label}</text>
            }
        })
        .collect();

    // X-axis time labels from the first series (show every Nth to avoid crowding)
    let x_labels: Vec<_> = if let Some(first) = series.first() {
        let step = (num_cols / 6).max(1); // at most ~6 labels
        first
            .points
            .iter()
            .enumerate()
            .filter(|(i, _)| i % step == 0 || *i == num_cols - 1)
            .map(|(i, p)| {
                let x = PAD_L + i as f64 * cell_w + cell_w / 2.0;
                let label = p.label.clone();
                view! {
                    <text
                        x=format!("{x:.1}")
                        y=format!("{:.1}", H - 6.0)
                        text-anchor="middle"
                        font-size="7"
                        class="fill-muted-foreground"
                    >{label}</text>
                }
            })
            .collect()
    } else {
        vec![]
    };

    // Axis lines (bottom + left), matching render_axes_and_grid style
    let bottom = PAD_T + plot_h;

    view! {
        <svg viewBox="0 0 320 200" class=svg_class aria-hidden="true">
            // Axis lines
            <line
                x1=PAD_L.to_string() y1=format!("{bottom:.1}")
                x2=(W - PAD_R).to_string() y2=format!("{bottom:.1}")
                stroke="currentColor" stroke-width="1" class="text-border"
            />
            <line
                x1=PAD_L.to_string() y1=PAD_T.to_string()
                x2=PAD_L.to_string() y2=format!("{bottom:.1}")
                stroke="currentColor" stroke-width="1" class="text-border"
            />
            {cells}
            {y_labels}
            {x_labels}
        </svg>
    }
    .into_any()
}
