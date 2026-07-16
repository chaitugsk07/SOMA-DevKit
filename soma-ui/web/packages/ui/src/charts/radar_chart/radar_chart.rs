use crate::charts::{ChartPoint, ChartSeries};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum RadarVariant {
    #[default]
    Default,
    Lines,
    Dots,
    Multiple,
}

#[component]
pub fn RadarChart(
    data: Vec<ChartPoint>,
    #[prop(default = RadarVariant::Default)] variant: RadarVariant,
    #[prop(optional, into)] series: Vec<ChartSeries>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    const CX: f64 = 110.0;
    const CY: f64 = 110.0;
    const R: f64 = 80.0;

    if data.is_empty() {
        return view! { <svg viewBox="0 0 220 220" class=format!("w-full h-auto {}", class)></svg> }.into_any();
    }

    let n = data.len();
    let max_val = data
        .iter()
        .map(|p| p.value)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let angle_of =
        |i: usize| 2.0 * std::f64::consts::PI * i as f64 / n as f64 - std::f64::consts::FRAC_PI_2;

    // Grid rings (3 rings)
    let rings: Vec<_> = (1..=3)
        .map(|k| {
            let r = R * k as f64 / 3.0;
            let ring_pts: String = (0..n)
                .map(|i| {
                    let a = angle_of(i);
                    format!("{:.1},{:.1}", CX + r * a.cos(), CY + r * a.sin())
                })
                .collect::<Vec<_>>()
                .join(" ");
            view! {
                <polygon points=ring_pts fill="none"
                    stroke="currentColor" stroke-width="0.5" class="text-border" />
            }
        })
        .collect();

    // Spokes
    let spokes: Vec<_> = (0..n)
        .map(|i| {
            let a = angle_of(i);
            let x = CX + R * a.cos();
            let y = CY + R * a.sin();
            view! {
                <line x1=format!("{CX:.1}") y1=format!("{CY:.1}")
                      x2=format!("{x:.1}") y2=format!("{y:.1}")
                      stroke="currentColor" stroke-width="0.5" class="text-border" />
            }
        })
        .collect();

    // Axis labels
    let axis_labels: Vec<_> = data
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let a = angle_of(i);
            let lx = CX + (R + 12.0) * a.cos();
            let ly = CY + (R + 12.0) * a.sin();
            let anchor = if a.cos() > 0.1 {
                "start"
            } else if a.cos() < -0.1 {
                "end"
            } else {
                "middle"
            };
            let label = p.label.clone();
            view! {
                <text x=format!("{lx:.1}") y=format!("{ly:.1}")
                    text-anchor=anchor font-size="8"
                    class="fill-muted-foreground">{label}</text>
            }
        })
        .collect();

    // Helper: build polygon points string from a slice
    let poly_pts = |pts: &[ChartPoint], max: f64| -> String {
        pts.iter()
            .enumerate()
            .map(|(i, p)| {
                let a = angle_of(i);
                let r = R * p.value / max;
                format!("{:.1},{:.1}", CX + r * a.cos(), CY + r * a.sin())
            })
            .collect::<Vec<_>>()
            .join(" ")
    };

    let palette_multi = [
        ("hsl(var(--primary))", "hsl(var(--primary) / 0.20)"),
        ("hsl(221 83% 53%)", "hsl(221 83% 53% / 0.20)"),
        ("hsl(262 80% 50%)", "hsl(262 80% 50% / 0.20)"),
        ("hsl(38 92% 50%)", "hsl(38 92% 50% / 0.20)"),
    ];

    let svg_content: Vec<_> = match &variant {
        RadarVariant::Default | RadarVariant::Lines => {
            let fill = if matches!(variant, RadarVariant::Default) {
                "hsl(var(--primary) / 0.25)"
            } else {
                "none"
            };
            let pts = poly_pts(&data, max_val);
            vec![view! {
                <polygon points=pts
                    fill=fill
                    stroke="hsl(var(--primary))"
                    stroke-width="2" stroke-linejoin="round" />
            }
            .into_any()]
        }
        RadarVariant::Dots => {
            let pts_str = poly_pts(&data, max_val);
            let dots: Vec<_> = data
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let a = angle_of(i);
                    let r = R * p.value / max_val;
                    let dx = CX + r * a.cos();
                    let dy = CY + r * a.sin();
                    view! {
                        <circle cx=format!("{dx:.1}") cy=format!("{dy:.1}") r="4"
                            fill="hsl(var(--primary))"
                            stroke="hsl(var(--background))" stroke-width="1.5" />
                    }
                    .into_any()
                })
                .collect();
            let mut out = vec![view! {
                <polygon points=pts_str
                    fill="hsl(var(--primary) / 0.20)"
                    stroke="hsl(var(--primary))"
                    stroke-width="2" stroke-linejoin="round" />
            }
            .into_any()];
            out.extend(dots);
            out
        }
        RadarVariant::Multiple => {
            let all_series: Vec<&[ChartPoint]> = if series.is_empty() {
                vec![&data]
            } else {
                series.iter().map(|s| s.points.as_slice()).collect()
            };

            let multi_max = all_series
                .iter()
                .flat_map(|s| s.iter().map(|p| p.value))
                .fold(0.0_f64, f64::max)
                .max(1.0);

            all_series
                .iter()
                .enumerate()
                .map(|(si, pts)| {
                    let (stroke, fill) = palette_multi[si % palette_multi.len()];
                    let poly = pts
                        .iter()
                        .enumerate()
                        .map(|(i, p)| {
                            let a = angle_of(i);
                            let r = R * p.value / multi_max;
                            format!("{:.1},{:.1}", CX + r * a.cos(), CY + r * a.sin())
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                    let stroke = stroke.to_string();
                    let fill = fill.to_string();
                    view! {
                        <polygon points=poly
                            fill=fill
                            stroke=stroke
                            stroke-width="2" stroke-linejoin="round" />
                    }
                    .into_any()
                })
                .collect()
        }
    };

    view! {
        <svg viewBox="0 0 220 220" class=format!("w-full h-auto {}", class) aria-hidden="true">
            {rings}
            {spokes}
            {svg_content}
            {axis_labels}
        </svg>
    }
    .into_any()
}
