use crate::charts::{
    linear_path, render_axes_and_grid, smooth_path, step_path, x_labels_view, ChartPoint,
    ChartSeries, CHART_PALETTE, H, PAD_B, PAD_L, PAD_R, PAD_T, W,
};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum AreaVariant {
    #[default]
    Default,
    Linear,
    Step,
    Gradient,
    Stacked,
}

fn x_of(i: usize, n: usize, plot_w: f64) -> f64 {
    PAD_L + (i as f64 / (n - 1).max(1) as f64) * plot_w
}

fn y_of(v: f64, max_val: f64, plot_h: f64) -> f64 {
    PAD_T + plot_h * (1.0 - v / max_val)
}

fn area_from_line(line_d: &str, pts: &[(f64, f64)], bottom: f64) -> String {
    format!(
        "{} L{:.1},{:.1} L{:.1},{:.1} Z",
        line_d,
        pts.last().unwrap().0,
        bottom,
        pts[0].0,
        bottom
    )
}

#[component]
pub fn AreaChart(
    data: Vec<ChartPoint>,
    #[prop(optional, into)] series: Vec<ChartSeries>,
    #[prop(default = AreaVariant::Default)] variant: AreaVariant,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    // Normalise: if series is empty, wrap `data` as one series.
    let all_series: Vec<ChartSeries> = if series.is_empty() {
        vec![ChartSeries {
            points: data.clone(),
        }]
    } else {
        series.clone()
    };

    let plot_w = W - PAD_L - PAD_R;
    let plot_h = H - PAD_T - PAD_B;
    let bottom = PAD_T + plot_h;

    if all_series.is_empty() || all_series[0].points.is_empty() {
        return view! {
            <svg viewBox="0 0 320 200" class=format!("w-full h-auto {}", class)></svg>
        }
        .into_any();
    }

    let n = all_series[0].points.len();

    // Max across all series (for Stacked: sum per category)
    let max_val = if matches!(variant, AreaVariant::Stacked) {
        (0..n)
            .map(|i| {
                all_series
                    .iter()
                    .map(|s| s.points.get(i).map(|p| p.value).unwrap_or(0.0))
                    .sum::<f64>()
            })
            .fold(0.0_f64, f64::max)
            .max(1.0)
    } else {
        all_series
            .iter()
            .flat_map(|s| s.points.iter().map(|p| p.value))
            .fold(0.0_f64, f64::max)
            .max(1.0)
    };

    let x_fn = move |i: usize| x_of(i, n, plot_w);
    let x_labels = x_labels_view(&all_series[0].points, &x_fn);

    let svg_content: Vec<_> = match variant {
        AreaVariant::Default | AreaVariant::Gradient => {
            let s = &all_series[0];
            let pts: Vec<(f64, f64)> = s
                .points
                .iter()
                .enumerate()
                .map(|(i, p)| (x_of(i, n, plot_w), y_of(p.value, max_val, plot_h)))
                .collect();
            let line_d = smooth_path(&pts);
            let area_d = area_from_line(&line_d, &pts, bottom);

            let fill = if matches!(variant, AreaVariant::Gradient) {
                "url(#area-gradient)".to_string()
            } else {
                "hsl(var(--primary) / 0.2)".to_string()
            };

            let gradient = if matches!(variant, AreaVariant::Gradient) {
                view! {
                    <defs>
                        <linearGradient id="area-gradient" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color="hsl(var(--primary))" stop-opacity="0.4"/>
                            <stop offset="100%" stop-color="hsl(var(--primary))" stop-opacity="0.0"/>
                        </linearGradient>
                    </defs>
                }
                .into_any()
            } else {
                view! { <></> }.into_any()
            };

            vec![view! {
                <>
                    {gradient}
                    {render_axes_and_grid()}
                    <path d=area_d fill=fill />
                    <path d=line_d fill="none" stroke="hsl(var(--primary))"
                        stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />
                    {x_labels}
                </>
            }
            .into_any()]
        }

        AreaVariant::Linear => {
            let s = &all_series[0];
            let pts: Vec<(f64, f64)> = s
                .points
                .iter()
                .enumerate()
                .map(|(i, p)| (x_of(i, n, plot_w), y_of(p.value, max_val, plot_h)))
                .collect();
            let line_d = linear_path(&pts);
            let area_d = area_from_line(&line_d, &pts, bottom);

            vec![view! {
                <>
                    {render_axes_and_grid()}
                    <path d=area_d fill="hsl(var(--primary) / 0.15)" />
                    <path d=line_d fill="none" stroke="hsl(var(--primary))"
                        stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />
                    {x_labels}
                </>
            }
            .into_any()]
        }

        AreaVariant::Step => {
            let s = &all_series[0];
            let pts: Vec<(f64, f64)> = s
                .points
                .iter()
                .enumerate()
                .map(|(i, p)| (x_of(i, n, plot_w), y_of(p.value, max_val, plot_h)))
                .collect();
            let line_d = step_path(&pts);
            let area_d = area_from_line(&line_d, &pts, bottom);

            vec![view! {
                <>
                    {render_axes_and_grid()}
                    <path d=area_d fill="hsl(var(--primary) / 0.15)" />
                    <path d=line_d fill="none" stroke="hsl(var(--primary))"
                        stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />
                    {x_labels}
                </>
            }
            .into_any()]
        }

        AreaVariant::Stacked => {
            let num_series = all_series.len();
            let mut stacked_views: Vec<AnyView> = Vec::with_capacity(num_series);

            // baseline[s][i] = sum of values from series 0..s at point i
            let mut baselines: Vec<Vec<f64>> = vec![vec![0.0; n]; num_series + 1];
            for si in 0..num_series {
                for i in 0..n {
                    let v = all_series[si].points.get(i).map(|p| p.value).unwrap_or(0.0);
                    baselines[si + 1][i] = baselines[si][i] + v;
                }
            }

            for si in 0..num_series {
                let color = CHART_PALETTE[si % CHART_PALETTE.len()];
                let top_pts: Vec<(f64, f64)> = (0..n)
                    .map(|i| {
                        (
                            x_of(i, n, plot_w),
                            y_of(baselines[si + 1][i], max_val, plot_h),
                        )
                    })
                    .collect();
                let bot_pts: Vec<(f64, f64)> = (0..n)
                    .map(|i| (x_of(i, n, plot_w), y_of(baselines[si][i], max_val, plot_h)))
                    .collect();

                // Area: top line forward, bottom line backward
                let mut area_d = format!("M{:.1},{:.1}", top_pts[0].0, top_pts[0].1);
                for (x, y) in top_pts.iter().skip(1) {
                    area_d.push_str(&format!(" L{x:.1},{y:.1}"));
                }
                for (x, y) in bot_pts.iter().rev() {
                    area_d.push_str(&format!(" L{x:.1},{y:.1}"));
                }
                area_d.push('Z');

                let line_d = linear_path(&top_pts);
                let color_owned = color.to_string();
                stacked_views.push(
                    view! {
                        <>
                            <path d=area_d fill=color_owned.clone() fill-opacity="0.25" />
                            <path d=line_d fill="none" stroke=color_owned stroke-width="2"
                                stroke-linejoin="round" stroke-linecap="round" />
                        </>
                    }
                    .into_any(),
                );
            }

            vec![view! {
                <>
                    {render_axes_and_grid()}
                    {stacked_views}
                    {x_labels}
                </>
            }
            .into_any()]
        }
    };

    view! {
        <svg viewBox="0 0 320 200" class=format!("w-full h-auto {}", class) aria-hidden="true">
            {svg_content}
        </svg>
    }
    .into_any()
}
