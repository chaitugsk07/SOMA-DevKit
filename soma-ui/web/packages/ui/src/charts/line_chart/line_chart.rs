use crate::charts::{
    linear_path, render_axes_and_grid, smooth_path, step_path, x_labels_view, ChartPoint,
    ChartSeries, CHART_PALETTE, H, PAD_B, PAD_L, PAD_R, PAD_T, W,
};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum LineVariant {
    #[default]
    Default,
    Linear,
    Step,
    Dots,
    Multiple,
}

impl std::str::FromStr for LineVariant {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Default" => Ok(Self::Default),
            "Linear" => Ok(Self::Linear),
            "Step" => Ok(Self::Step),
            "Dots" => Ok(Self::Dots),
            "Multiple" => Ok(Self::Multiple),
            _ => Err(()),
        }
    }
}

fn plot_coords(data: &[ChartPoint]) -> (impl Fn(usize) -> f64 + '_, impl Fn(f64) -> f64 + '_) {
    let plot_w = W - PAD_L - PAD_R;
    let plot_h = H - PAD_T - PAD_B;
    let max_val = data
        .iter()
        .map(|p| p.value)
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let n = data.len();
    let x_of = move |i: usize| PAD_L + (i as f64 / (n - 1).max(1) as f64) * plot_w;
    let y_of = move |v: f64| PAD_T + plot_h * (1.0 - v / max_val);
    (x_of, y_of)
}

#[component]
pub fn LineChart(
    #[prop(default = vec![])] data: Vec<ChartPoint>,
    #[prop(default = LineVariant::Default)] variant: LineVariant,
    #[prop(optional, into)] series: Vec<ChartSeries>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    // For Multiple, use series; otherwise use data (wrap if series provided but variant isn't Multiple)
    let effective_data: Vec<ChartPoint> = if data.is_empty() && !series.is_empty() {
        series[0].points.clone()
    } else {
        data.clone()
    };

    if matches!(variant, LineVariant::Multiple) {
        let all_series = if series.is_empty() {
            vec![ChartSeries {
                points: effective_data.clone(),
            }]
        } else {
            series.clone()
        };

        let ref_series = all_series[0].points.clone();
        if ref_series.is_empty() {
            return view! { <svg viewBox="0 0 320 200" class=format!("w-full h-auto {}", class)></svg> }.into_any();
        }

        let max_val = all_series
            .iter()
            .flat_map(|s| s.points.iter().map(|p| p.value))
            .fold(0.0_f64, f64::max)
            .max(1.0);
        let plot_w = W - PAD_L - PAD_R;
        let plot_h = H - PAD_T - PAD_B;
        let n = ref_series.len();
        let x_of = move |i: usize| PAD_L + (i as f64 / (n - 1).max(1) as f64) * plot_w;
        let y_of = move |v: f64| PAD_T + plot_h * (1.0 - v / max_val);

        let series_paths: Vec<_> = all_series
            .iter()
            .enumerate()
            .map(|(si, s)| {
                let pts: Vec<(f64, f64)> = s
                    .points
                    .iter()
                    .enumerate()
                    .map(|(i, p)| (x_of(i), y_of(p.value)))
                    .collect();
                let d = smooth_path(&pts);
                let color = CHART_PALETTE[si % CHART_PALETTE.len()].to_string();
                view! {
                    <path d=d fill="none" stroke=color stroke-width="2"
                        stroke-linejoin="round" stroke-linecap="round" />
                }
            })
            .collect();

        let labels = x_labels_view(&ref_series, &x_of);

        return view! {
            <svg viewBox="0 0 320 200" class=format!("w-full h-auto {}", class) aria-hidden="true">
                {render_axes_and_grid()}
                {series_paths}
                {labels}
            </svg>
        }
        .into_any();
    }

    // Single-series variants
    if effective_data.is_empty() {
        return view! { <svg viewBox="0 0 320 200" class=format!("w-full h-auto {}", class)></svg> }.into_any();
    }

    let (x_of, y_of) = plot_coords(&effective_data);
    let pts: Vec<(f64, f64)> = effective_data
        .iter()
        .enumerate()
        .map(|(i, p)| (x_of(i), y_of(p.value)))
        .collect();

    let line_element: Vec<_> = match variant {
        LineVariant::Default => {
            let d = smooth_path(&pts);
            vec![view! {
                <path d=d fill="none" stroke="hsl(var(--primary))"
                    stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />
            }
            .into_any()]
        }
        LineVariant::Linear => {
            let d = linear_path(&pts);
            vec![view! {
                <path d=d fill="none" stroke="hsl(var(--primary))"
                    stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />
            }
            .into_any()]
        }
        LineVariant::Step => {
            let d = step_path(&pts);
            vec![view! {
                <path d=d fill="none" stroke="hsl(var(--primary))"
                    stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />
            }
            .into_any()]
        }
        LineVariant::Dots => {
            let d = linear_path(&pts);
            let dots: Vec<_> = pts
                .iter()
                .map(|(x, y)| {
                    view! {
                        <circle cx=format!("{x:.1}") cy=format!("{y:.1}") r="3.5"
                            fill="hsl(var(--primary))" />
                    }
                })
                .collect();
            vec![
                view! {
                    <path d=d fill="none" stroke="hsl(var(--primary))"
                        stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />
                }
                .into_any(),
                view! { <g>{dots}</g> }.into_any(),
            ]
        }
        LineVariant::Multiple => unreachable!(),
    };

    let labels = x_labels_view(&effective_data, &x_of);

    view! {
        <svg viewBox="0 0 320 200" class=format!("w-full h-auto {}", class) aria-hidden="true">
            {render_axes_and_grid()}
            {line_element}
            {labels}
        </svg>
    }
    .into_any()
}
