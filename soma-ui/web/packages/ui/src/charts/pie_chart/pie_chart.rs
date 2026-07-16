use crate::charts::ChartPoint;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum PieVariant {
    #[default]
    Pie,
    Donut,
    Labeled,
    Half,
}

impl std::str::FromStr for PieVariant {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pie" => Ok(Self::Pie),
            "Donut" => Ok(Self::Donut),
            "Labeled" => Ok(Self::Labeled),
            "Half" => Ok(Self::Half),
            _ => Err(()),
        }
    }
}

const CX: f64 = 100.0;
const CY: f64 = 100.0;
const R: f64 = 75.0;
const R_INNER: f64 = 42.0; // donut hole radius

const PALETTE: [&str; 6] = [
    "hsl(var(--primary))",
    "hsl(var(--destructive))",
    "hsl(221 83% 53%)",
    "hsl(262 80% 50%)",
    "hsl(38 92% 50%)",
    "hsl(160 60% 45%)",
];

fn arc_path(cx: f64, cy: f64, r: f64, start: f64, end: f64) -> (f64, f64, f64, f64, i32) {
    let x1 = cx + r * start.cos();
    let y1 = cy + r * start.sin();
    let x2 = cx + r * end.cos();
    let y2 = cy + r * end.sin();
    let large = if (end - start).abs() > std::f64::consts::PI {
        1
    } else {
        0
    };
    (x1, y1, x2, y2, large)
}

/// Build slices: returns Vec<(path_d, fill, mid_angle, label, frac)>
fn build_slices(
    data: &[ChartPoint],
    start_angle: f64,
    sweep_total: f64,
) -> Vec<(String, String, f64, String, f64)> {
    let total: f64 = data.iter().map(|p| p.value).sum::<f64>().max(1.0);
    let mut angle = start_angle;
    data.iter()
        .enumerate()
        .map(|(i, p)| {
            let frac = p.value / total;
            let sweep = frac * sweep_total;
            let end = angle + sweep;
            let (x1, y1, x2, y2, large) = arc_path(CX, CY, R, angle, end);
            let path = format!(
                "M{CX:.1},{CY:.1} L{x1:.1},{y1:.1} A{R:.1},{R:.1} 0 {large},1 {x2:.1},{y2:.1} Z"
            );
            let mid = angle + sweep / 2.0;
            let fill = PALETTE[i % PALETTE.len()].to_string();
            angle = end;
            (path, fill, mid, p.label.clone(), frac)
        })
        .collect()
}

/// Build donut slices (annular sector path)
fn build_donut_slices(
    data: &[ChartPoint],
    start_angle: f64,
    sweep_total: f64,
) -> Vec<(String, String)> {
    let total: f64 = data.iter().map(|p| p.value).sum::<f64>().max(1.0);
    let mut angle = start_angle;
    data.iter()
        .enumerate()
        .map(|(i, p)| {
            let frac = p.value / total;
            let sweep = frac * sweep_total;
            let end = angle + sweep;

            let (ox1, oy1, ox2, oy2, large) = arc_path(CX, CY, R, angle, end);
            let (ix1, iy1, ix2, iy2, _) = arc_path(CX, CY, R_INNER, angle, end);
            // outer arc CW, inner arc CCW
            let path = format!(
                "M{ox1:.1},{oy1:.1} A{R:.1},{R:.1} 0 {large},1 {ox2:.1},{oy2:.1} \
             L{ix2:.1},{iy2:.1} A{R_INNER:.1},{R_INNER:.1} 0 {large},0 {ix1:.1},{iy1:.1} Z"
            );
            let fill = PALETTE[i % PALETTE.len()].to_string();
            angle = end;
            (path, fill)
        })
        .collect()
}

#[component]
pub fn PieChart(
    #[prop(default = vec![])] data: Vec<ChartPoint>,
    #[prop(default = PieVariant::Pie)] variant: PieVariant,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    if data.is_empty() {
        return view! { <svg viewBox="0 0 200 200" class=format!("w-full h-auto {}", class)></svg> }.into_any();
    }

    let pi = std::f64::consts::PI;
    let start = -pi / 2.0; // top

    match variant {
        PieVariant::Pie => {
            let slices: Vec<_> = build_slices(&data, start, 2.0 * pi)
                .into_iter()
                .map(|(d, fill, _, _, _)| {
                    view! {
                        <path d=d fill=fill stroke="hsl(var(--background))" stroke-width="1.5" />
                    }
                })
                .collect();
            view! {
                <svg viewBox="0 0 200 200" class=format!("w-full h-auto {}", class) aria-hidden="true">
                    {slices}
                </svg>
            }.into_any()
        }

        PieVariant::Donut => {
            let slices: Vec<_> = build_donut_slices(&data, start, 2.0 * pi)
                .into_iter()
                .map(|(d, fill)| {
                    view! {
                        <path d=d fill=fill stroke="hsl(var(--background))" stroke-width="1.5" />
                    }
                })
                .collect();
            view! {
                <svg viewBox="0 0 200 200" class=format!("w-full h-auto {}", class) aria-hidden="true">
                    {slices}
                </svg>
            }.into_any()
        }

        PieVariant::Labeled => {
            let total: f64 = data.iter().map(|p| p.value).sum::<f64>();
            let total_str = format!("{:.0}", total);
            let donut_slices: Vec<_> = build_donut_slices(&data, start, 2.0 * pi)
                .into_iter()
                .map(|(d, fill)| {
                    view! {
                        <path d=d fill=fill stroke="hsl(var(--background))" stroke-width="1.5" />
                    }
                })
                .collect();
            // Slice percentage labels at mid-angle
            let label_elems: Vec<_> = build_slices(&data, start, 2.0 * pi)
                .into_iter()
                .filter(|(_, _, _, _, frac)| *frac >= 0.08) // skip tiny slices
                .map(|(_, _, mid, _, frac)| {
                    let lx = CX + (R + R_INNER) / 2.0 * mid.cos();
                    let ly = CY + (R + R_INNER) / 2.0 * mid.sin();
                    let pct = format!("{:.0}%", frac * 100.0);
                    view! {
                        <text x=format!("{lx:.1}") y=format!("{ly:.1}")
                            text-anchor="middle" dominant-baseline="middle"
                            font-size="9" fill="white" font-weight="600">{pct}</text>
                    }
                })
                .collect();
            view! {
                <svg viewBox="0 0 200 200" class=format!("w-full h-auto {}", class) aria-hidden="true">
                    {donut_slices}
                    // Centered total
                    <text x=format!("{CX:.1}") y=format!("{:.1}", CY - 6.0)
                        text-anchor="middle" dominant-baseline="middle"
                        font-size="18" class="fill-foreground" font-weight="700">{total_str}</text>
                    <text x=format!("{CX:.1}") y=format!("{:.1}", CY + 10.0)
                        text-anchor="middle" dominant-baseline="middle"
                        font-size="9" class="fill-muted-foreground">{"Total"}</text>
                    {label_elems}
                </svg>
            }.into_any()
        }

        PieVariant::Half => {
            // Semi-circle gauge: sweep PI (180°), start at left (-PI), end at right (0)
            let half_start = pi; // left side
            let donut_slices: Vec<_> = build_donut_slices(&data, half_start, pi)
                .into_iter()
                .map(|(d, fill)| {
                    view! {
                        <path d=d fill=fill stroke="hsl(var(--background))" stroke-width="1.5" />
                    }
                })
                .collect();
            // viewBox shifts so semi-circle fits: show top half only (CY=100, R=75 → top at 25)
            view! {
                <svg viewBox="0 0 200 120" class=format!("w-full h-auto {}", class) aria-hidden="true">
                    {donut_slices}
                </svg>
            }.into_any()
        }
    }
}
