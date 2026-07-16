use crate::charts::{ChartPoint, ChartSeries, CHART_PALETTE};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum RadialVariant {
    #[default]
    Default,
    Stacked,
    Labeled,
    Track,
}

#[component]
pub fn RadialChart(
    data: Vec<ChartPoint>,
    #[prop(default = RadialVariant::Default)] variant: RadialVariant,
    #[prop(optional, into)] series: Vec<ChartSeries>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    const CX: f64 = 110.0;
    const CY: f64 = 110.0;
    const TRACK_W: f64 = 10.0;
    const GAP: f64 = 4.0;

    if data.is_empty() {
        return view! { <svg viewBox="0 0 220 220" class=format!("w-full h-auto {}", class)></svg> }.into_any();
    }

    let max_val = data
        .iter()
        .map(|p| p.value)
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let outer_r = CX - 10.0;

    match &variant {
        RadialVariant::Default | RadialVariant::Track | RadialVariant::Labeled => {
            // Default: opacity 0.3, round linecap
            // Track:   opacity 0.5, butt  linecap
            // Labeled: same as Default but adds center text
            let is_track = matches!(variant, RadialVariant::Track);
            let track_opacity = if is_track { "0.5" } else { "0.3" };
            let linecap = if is_track { "butt" } else { "round" };

            let arcs: Vec<_> =
                data.iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let r = outer_r - i as f64 * (TRACK_W + GAP);
                        if r < 10.0 {
                            return view! { <g></g> }.into_any();
                        }
                        let frac = p.value / max_val;
                        let circ = 2.0 * std::f64::consts::PI * r;
                        let dash = frac * circ;
                        let gap = circ - dash;
                        let color = CHART_PALETTE[i % CHART_PALETTE.len()].to_string();
                        view! {
                    <circle cx=format!("{CX:.1}") cy=format!("{CY:.1}") r=format!("{r:.1}")
                        fill="none" stroke="hsl(var(--muted))" stroke-width=TRACK_W.to_string()
                        opacity=track_opacity />
                    <circle cx=format!("{CX:.1}") cy=format!("{CY:.1}") r=format!("{r:.1}")
                        fill="none" stroke=color stroke-width=TRACK_W.to_string()
                        stroke-dasharray=format!("{dash:.1} {gap:.1}")
                        stroke-linecap=linecap
                        transform=format!("rotate(-90 {CX:.1} {CY:.1})") />
                }.into_any()
                    })
                    .collect();

            if matches!(variant, RadialVariant::Labeled) {
                let total: f64 = data.iter().map(|p| p.value).sum();
                let total_str = format!("{:.0}", total);
                view! {
                    <svg viewBox="0 0 220 220" class=format!("w-full h-auto {}", class) aria-hidden="true">
                        {arcs}
                        <text x=format!("{CX:.1}") y=format!("{:.1}", CY - 6.0)
                            text-anchor="middle" font-size="22" font-weight="700"
                            class="fill-foreground">{total_str}</text>
                        <text x=format!("{CX:.1}") y=format!("{:.1}", CY + 12.0)
                            text-anchor="middle" font-size="9"
                            class="fill-muted-foreground">"total"</text>
                    </svg>
                }.into_any()
            } else {
                view! {
                    <svg viewBox="0 0 220 220" class=format!("w-full h-auto {}", class) aria-hidden="true">
                        {arcs}
                    </svg>
                }.into_any()
            }
        }

        RadialVariant::Stacked => {
            let all_series: Vec<&Vec<ChartPoint>> = if series.is_empty() {
                vec![&data]
            } else {
                series.iter().map(|s| &s.points).collect()
            };

            let n_rings = all_series.first().map(|s| s.len()).unwrap_or(0);

            let ring_totals: Vec<f64> = (0..n_rings)
                .map(|ri| {
                    all_series
                        .iter()
                        .map(|s| s.get(ri).map(|p| p.value).unwrap_or(0.0))
                        .sum::<f64>()
                        .max(1.0)
                })
                .collect();

            let arcs: Vec<_> = (0..n_rings)
                .flat_map(|ri| {
                    let r = outer_r - ri as f64 * (TRACK_W + GAP);
                    if r < 10.0 {
                        return vec![view! { <g></g> }.into_any()];
                    }
                    let total = ring_totals[ri];
                    let circ = 2.0 * std::f64::consts::PI * r;

                    let mut out = vec![view! {
                        <circle cx=format!("{CX:.1}") cy=format!("{CY:.1}") r=format!("{r:.1}")
                            fill="none" stroke="hsl(var(--muted))" stroke-width=TRACK_W.to_string()
                            opacity="0.3" />
                    }
                    .into_any()];

                    let mut used_frac = 0.0_f64;
                    for (si, s) in all_series.iter().enumerate() {
                        let val = s.get(ri).map(|p| p.value).unwrap_or(0.0);
                        let frac = val / total;
                        let seg = frac * circ;
                        let offset = used_frac * circ;
                        let gap = circ - seg;
                        let color = CHART_PALETTE[si % CHART_PALETTE.len()].to_string();
                        used_frac += frac;
                        out.push(view! {
                        <circle cx=format!("{CX:.1}") cy=format!("{CY:.1}") r=format!("{r:.1}")
                            fill="none" stroke=color stroke-width=TRACK_W.to_string()
                            stroke-dasharray=format!("{seg:.1} {gap:.1}")
                            stroke-dashoffset=format!("{:.1}", -offset)
                            stroke-linecap="butt"
                            transform=format!("rotate(-90 {CX:.1} {CY:.1})") />
                    }.into_any());
                    }
                    out
                })
                .collect();

            view! {
                <svg viewBox="0 0 220 220" class=format!("w-full h-auto {}", class) aria-hidden="true">
                    {arcs}
                </svg>
            }.into_any()
        }
    }
}
