use crate::charts::ChartPoint;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SparkVariant {
    #[default]
    Line,
    Area,
    Bars,
}

/// Minimal axis-less trend line for embedding in table rows or stat tiles.
///
/// Renders an SVG that fills its container (`width="100%" height="100%"`).
/// ViewBox is `0 0 100 28`; `preserveAspectRatio="none"`.
/// Empty data → empty `<svg>` with no rendered content.
#[component]
pub fn Sparkline(
    #[prop(default = vec![])] data: Vec<ChartPoint>,
    #[prop(default = SparkVariant::Line)] variant: SparkVariant,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    const VW: f64 = 100.0;
    const VH: f64 = 28.0;
    const PAD: f64 = 1.5; // tiny inset so strokes aren't clipped

    let svg_class = format!("w-full h-full {}", class);

    if data.is_empty() {
        return view! {
            <svg viewBox="0 0 100 28" preserveAspectRatio="none"
                 width="100%" height="100%"
                 class=svg_class aria-hidden="true" />
        }
        .into_any();
    }

    let min_val = data.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
    let max_val = data
        .iter()
        .map(|p| p.value)
        .fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(1.0);

    let n = data.len();
    let plot_w = VW - PAD * 2.0;
    let plot_h = VH - PAD * 2.0;

    let x_of = |i: usize| PAD + (i as f64 / (n - 1).max(1) as f64) * plot_w;
    let y_of = |v: f64| PAD + plot_h * (1.0 - (v - min_val) / range);

    let content: AnyView = match variant {
        SparkVariant::Line => {
            let pts: Vec<(f64, f64)> = data
                .iter()
                .enumerate()
                .map(|(i, p)| (x_of(i), y_of(p.value)))
                .collect();
            // Simple polyline path
            let d = {
                let mut s = format!("M{:.2},{:.2}", pts[0].0, pts[0].1);
                for (x, y) in pts.iter().skip(1) {
                    s.push_str(&format!(" L{x:.2},{y:.2}"));
                }
                s
            };
            view! {
                <path d=d fill="none" stroke="hsl(var(--primary))"
                    stroke-width="1.5" stroke-linejoin="round" stroke-linecap="round" />
            }
            .into_any()
        }
        SparkVariant::Area => {
            let pts: Vec<(f64, f64)> = data
                .iter()
                .enumerate()
                .map(|(i, p)| (x_of(i), y_of(p.value)))
                .collect();
            let baseline_y = PAD + plot_h; // bottom of plot area
            let first_x = pts[0].0;
            let last_x = pts[pts.len() - 1].0;
            // Build line path
            let mut line_d = format!("M{:.2},{:.2}", pts[0].0, pts[0].1);
            for (x, y) in pts.iter().skip(1) {
                line_d.push_str(&format!(" L{x:.2},{y:.2}"));
            }
            // Area path: line + close to baseline
            let area_d = format!(
                "{} L{:.2},{:.2} L{:.2},{:.2} Z",
                line_d, last_x, baseline_y, first_x, baseline_y
            );
            view! {
                <>
                    <path d=area_d fill="hsl(var(--primary))" fill-opacity="0.15" stroke="none" />
                    <path d=line_d fill="none" stroke="hsl(var(--primary))"
                        stroke-width="1.5" stroke-linejoin="round" stroke-linecap="round" />
                </>
            }
            .into_any()
        }
        SparkVariant::Bars => {
            // Bar charts always baseline from 0 so heights are proportional to
            // absolute values. Using (value - min_val)/range would make the
            // smallest non-zero bucket invisible while the next-smallest
            // appears full-height — especially visible with sparse audit data.
            let bar_max = max_val.max(1.0);
            let bar_slot = plot_w / n as f64;
            let bar_w = (bar_slot * 0.7).max(1.0);
            let gap = (bar_slot - bar_w) / 2.0;
            let baseline_y = PAD + plot_h;
            let bars: Vec<_> = data
                .iter()
                .enumerate()
                .filter(|(_, p)| p.value > 0.0)
                .map(|(i, p)| {
                    // Clamp to 0.5 minimum so any non-zero bucket is visible.
                    let bar_h = (p.value / bar_max * plot_h).max(0.5);
                    let x = PAD + i as f64 * bar_slot + gap;
                    let y = baseline_y - bar_h;
                    view! {
                        <rect x=format!("{x:.2}") y=format!("{y:.2}")
                            width=format!("{bar_w:.2}") height=format!("{bar_h:.2}")
                            fill="hsl(var(--primary))" rx="0.5" />
                    }
                })
                .collect();
            view! { <g>{bars}</g> }.into_any()
        }
    };

    view! {
        <svg viewBox="0 0 100 28" preserveAspectRatio="none"
             width="100%" height="100%"
             class=svg_class aria-hidden="true">
            {content}
        </svg>
    }
    .into_any()
}
