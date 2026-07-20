use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ChartPoint {
    pub label: String,
    pub value: f64,
}

/// A named series for multi-series chart variants (stacked / grouped / multiple).
/// `points` are aligned by index across series (shared category axis).
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
    pub points: Vec<ChartPoint>,
}

// Shared geometry for the 320×200 Cartesian charts (area / bar / line).
pub(crate) const W: f64 = 320.0;
pub(crate) const H: f64 = 200.0;
pub(crate) const PAD_L: f64 = 30.0;
pub(crate) const PAD_B: f64 = 30.0;
pub(crate) const PAD_T: f64 = 10.0;
pub(crate) const PAD_R: f64 = 10.0;

/// Shared color palette for multi-series charts.
pub const CHART_PALETTE: &[&str] = &[
    "hsl(var(--primary))",
    "hsl(var(--chart-2, 200 70% 55%))",
    "hsl(var(--chart-3, 150 60% 50%))",
    "hsl(var(--chart-4, 30 80% 55%))",
    "hsl(var(--destructive))",
    "hsl(262 80% 50%)",
];

/// Build a smooth cubic-bezier path.
pub(crate) fn smooth_path(pts: &[(f64, f64)]) -> String {
    if pts.len() < 2 {
        return pts
            .first()
            .map(|(x, y)| format!("M{x:.1},{y:.1}"))
            .unwrap_or_default();
    }
    let mut d = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for i in 1..pts.len() {
        let (x0, y0) = pts[i - 1];
        let (x1, y1) = pts[i];
        let cx = (x0 + x1) / 2.0;
        d.push_str(&format!(
            " C{cx:.1},{y0:.1} {cx:.1},{y1:.1} {x1:.1},{y1:.1}"
        ));
    }
    d
}

/// Build a straight-segment polyline path.
pub(crate) fn linear_path(pts: &[(f64, f64)]) -> String {
    let mut d = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for (x, y) in pts.iter().skip(1) {
        d.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    d
}

/// Build a step path (horizontal first, then vertical).
pub(crate) fn step_path(pts: &[(f64, f64)]) -> String {
    if pts.is_empty() {
        return String::new();
    }
    let mut d = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for i in 1..pts.len() {
        let (x1, y1) = pts[i];
        d.push_str(&format!(" H{x1:.1} V{y1:.1}"));
    }
    d
}

/// Shared axes + dashed gridlines renderer for the 320×200 Cartesian charts.
pub(crate) fn render_axes_and_grid() -> impl IntoView {
    let plot_h = H - PAD_T - PAD_B;
    let bottom = PAD_T + plot_h;
    let gridlines: Vec<_> = (1..=4)
        .map(|i| {
            let y = PAD_T + plot_h * (i as f64 / 4.0);
            view! {
                <line
                    x1=PAD_L.to_string() y1=format!("{y:.1}")
                    x2=(W - PAD_R).to_string() y2=format!("{y:.1}")
                    stroke="currentColor" stroke-width="0.5"
                    class="text-border" stroke-dasharray="3,3"
                />
            }
        })
        .collect();
    view! {
        {gridlines}
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
    }
}

/// Render x-axis category labels below the chart.
pub(crate) fn x_labels_view(
    data: &[ChartPoint],
    x_of: &impl Fn(usize) -> f64,
) -> Vec<impl IntoView> {
    data.iter()
        .enumerate()
        .map(|(i, p)| {
            let x = x_of(i);
            let label = p.label.clone();
            view! {
                <text x=format!("{x:.1}") y=format!("{:.1}", H - 6.0)
                    text-anchor="middle" font-size="8"
                    class="fill-muted-foreground">{label}</text>
            }
        })
        .collect()
}

pub mod area_chart;
pub use area_chart::*;
pub mod bar_chart;
pub use bar_chart::*;
pub mod heatmap;
pub use heatmap::*;
pub mod line_chart;
pub use line_chart::*;
pub mod pie_chart;
pub use pie_chart::*;
pub mod radar_chart;
pub use radar_chart::*;
pub mod radial_chart;
pub use radial_chart::*;
pub mod sparkline;
pub use sparkline::*;
pub mod stat_tile;
pub use stat_tile::*;
pub mod security_graph;
pub use security_graph::*;
pub mod donut_stat;
pub use donut_stat::*;
pub mod risk_gauge;
pub use risk_gauge::*;
