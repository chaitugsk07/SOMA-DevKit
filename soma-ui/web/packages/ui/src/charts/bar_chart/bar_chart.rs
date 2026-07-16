use crate::charts::{
    render_axes_and_grid, ChartPoint, ChartSeries, CHART_PALETTE, H, PAD_B, PAD_L, PAD_R, PAD_T, W,
};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BarVariant {
    #[default]
    Default,
    Horizontal,
    Stacked,
    Grouped,
}

#[component]
pub fn BarChart(
    data: Vec<ChartPoint>,
    #[prop(optional, into)] series: Vec<ChartSeries>,
    #[prop(default = BarVariant::Default)] variant: BarVariant,
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
    let num_series = all_series.len();

    let content: AnyView =
        match variant {
            BarVariant::Default => {
                let s = &all_series[0];
                let max_val = s
                    .points
                    .iter()
                    .map(|p| p.value)
                    .fold(0.0_f64, f64::max)
                    .max(1.0);
                let slot = plot_w / n as f64;
                let bar_w = slot * 0.6;
                let gap = slot * 0.2;

                let bars: Vec<_> = s
                    .points
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let bar_h = (p.value / max_val) * plot_h;
                        let x = PAD_L + gap + i as f64 * slot;
                        let y = bottom - bar_h;
                        let label = p.label.clone();
                        view! {
                            <>
                                <rect x=format!("{x:.1}") y=format!("{y:.1}")
                                    width=format!("{bar_w:.1}") height=format!("{bar_h:.1}")
                                    fill="hsl(var(--primary))" rx="2" />
                                <text x=format!("{:.1}", x + bar_w / 2.0)
                                    y=format!("{:.1}", H - 6.0)
                                    text-anchor="middle" font-size="8"
                                    class="fill-muted-foreground">{label}</text>
                            </>
                        }
                    })
                    .collect();

                view! {
                    <>
                        {render_axes_and_grid()}
                        {bars}
                    </>
                }
                .into_any()
            }

            BarVariant::Horizontal => {
                let s = &all_series[0];
                let max_val = s
                    .points
                    .iter()
                    .map(|p| p.value)
                    .fold(0.0_f64, f64::max)
                    .max(1.0);
                let slot = plot_h / n as f64;
                let bar_h = slot * 0.6;
                let gap = slot * 0.2;

                // Vertical gridlines on the value (x) axis
                let h_gridlines: Vec<_> = (1..=4)
                    .map(|i| {
                        let x = PAD_L + plot_w * (i as f64 / 4.0);
                        view! {
                            <line
                                x1=format!("{x:.1}") y1=PAD_T.to_string()
                                x2=format!("{x:.1}") y2=format!("{bottom:.1}")
                                stroke="currentColor" stroke-width="0.5"
                                class="text-border" stroke-dasharray="3,3"
                            />
                        }
                    })
                    .collect();

                let bars: Vec<_> = s
                    .points
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let bar_len = (p.value / max_val) * plot_w;
                        let y = PAD_T + gap + i as f64 * slot;
                        let label = p.label.clone();
                        view! {
                            <>
                                <rect x=format!("{PAD_L:.1}") y=format!("{y:.1}")
                                    width=format!("{bar_len:.1}") height=format!("{bar_h:.1}")
                                    fill="hsl(var(--primary))" rx="2" />
                                <text x=format!("{:.1}", PAD_L - 3.0)
                                    y=format!("{:.1}", y + bar_h / 2.0 + 3.0)
                                    text-anchor="end" font-size="8"
                                    class="fill-muted-foreground">{label}</text>
                            </>
                        }
                    })
                    .collect();

                view! {
                    <>
                        {h_gridlines}
                        <line x1=PAD_L.to_string() y1=format!("{bottom:.1}")
                              x2=(W - PAD_R).to_string() y2=format!("{bottom:.1}")
                              stroke="currentColor" stroke-width="1" class="text-border" />
                        <line x1=PAD_L.to_string() y1=PAD_T.to_string()
                              x2=PAD_L.to_string() y2=format!("{bottom:.1}")
                              stroke="currentColor" stroke-width="1" class="text-border" />
                        {bars}
                    </>
                }
                .into_any()
            }

            BarVariant::Stacked => {
                let max_val = (0..n)
                    .map(|i| {
                        all_series
                            .iter()
                            .map(|s| s.points.get(i).map(|p| p.value).unwrap_or(0.0))
                            .sum::<f64>()
                    })
                    .fold(0.0_f64, f64::max)
                    .max(1.0);

                let slot = plot_w / n as f64;
                let bar_w = slot * 0.6;
                let gap = slot * 0.2;

                let mut all_rects: Vec<AnyView> = Vec::new();

                let x_labels: Vec<_> = all_series[0]
                    .points
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let x = PAD_L + gap + i as f64 * slot;
                        let label = p.label.clone();
                        view! {
                            <text x=format!("{:.1}", x + bar_w / 2.0)
                                y=format!("{:.1}", H - 6.0)
                                text-anchor="middle" font-size="8"
                                class="fill-muted-foreground">{label}</text>
                        }
                    })
                    .collect();

                for i in 0..n {
                    let mut y_cursor = bottom;
                    for si in 0..num_series {
                        let v = all_series[si].points.get(i).map(|p| p.value).unwrap_or(0.0);
                        let bar_h = (v / max_val) * plot_h;
                        let x = PAD_L + gap + i as f64 * slot;
                        let y = y_cursor - bar_h;
                        let color = CHART_PALETTE[si % CHART_PALETTE.len()].to_string();
                        all_rects.push(
                            view! {
                                <rect x=format!("{x:.1}") y=format!("{y:.1}")
                                    width=format!("{bar_w:.1}") height=format!("{bar_h:.1}")
                                    fill=color rx="2" />
                            }
                            .into_any(),
                        );
                        y_cursor = y;
                    }
                }

                view! {
                    <>
                        {render_axes_and_grid()}
                        {all_rects}
                        {x_labels}
                    </>
                }
                .into_any()
            }

            BarVariant::Grouped => {
                let max_val = all_series
                    .iter()
                    .flat_map(|s| s.points.iter().map(|p| p.value))
                    .fold(0.0_f64, f64::max)
                    .max(1.0);

                let slot = plot_w / n as f64;
                let group_w = slot * 0.8;
                let group_gap = slot * 0.1;
                let sub_w = group_w / num_series as f64;

                let mut all_rects: Vec<AnyView> = Vec::new();

                let x_labels: Vec<_> = all_series[0]
                    .points
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let group_x = PAD_L + group_gap + i as f64 * slot;
                        let label = p.label.clone();
                        view! {
                            <text x=format!("{:.1}", group_x + group_w / 2.0)
                                y=format!("{:.1}", H - 6.0)
                                text-anchor="middle" font-size="8"
                                class="fill-muted-foreground">{label}</text>
                        }
                    })
                    .collect();

                for i in 0..n {
                    let group_x = PAD_L + group_gap + i as f64 * slot;
                    for si in 0..num_series {
                        let v = all_series[si].points.get(i).map(|p| p.value).unwrap_or(0.0);
                        let bar_h = (v / max_val) * plot_h;
                        let x = group_x + si as f64 * sub_w;
                        let y = bottom - bar_h;
                        let color = CHART_PALETTE[si % CHART_PALETTE.len()].to_string();
                        all_rects.push(view! {
                        <rect x=format!("{x:.1}") y=format!("{y:.1}")
                            width=format!("{:.1}", sub_w * 0.85) height=format!("{bar_h:.1}")
                            fill=color rx="2" />
                    }.into_any());
                    }
                }

                view! {
                    <>
                        {render_axes_and_grid()}
                        {all_rects}
                        {x_labels}
                    </>
                }
                .into_any()
            }
        };

    view! {
        <svg viewBox="0 0 320 200" class=format!("w-full h-auto {}", class) aria-hidden="true">
            {content}
        </svg>
    }
    .into_any()
}
