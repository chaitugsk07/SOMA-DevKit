use leptos::prelude::*;

/// One segment in a DonutStat chart.
#[derive(Clone, Debug)]
pub struct DonutItem {
    pub label: String,
    pub value: u32,
    /// Hex color for this segment (e.g. `"#e5484d"` for Critical).
    pub color: String,
}

const CX: f64 = 60.0;
const CY: f64 = 60.0;
const R: f64 = 46.0;
const R_INNER: f64 = 28.0;

fn arc_path(cx: f64, cy: f64, r: f64, angle: f64, end: f64) -> (f64, f64, f64, f64, i32) {
    let x1 = cx + r * angle.cos();
    let y1 = cy + r * angle.sin();
    let x2 = cx + r * end.cos();
    let y2 = cy + r * end.sin();
    let large = if (end - angle).abs() > std::f64::consts::PI { 1 } else { 0 };
    (x1, y1, x2, y2, large)
}

/// Donut chart with a centered total and a colored legend.
/// Use `DonutItem.color` directly (pass severity hex values from `Severity::hex()`).
#[component]
pub fn DonutStat(
    #[prop(default = vec![])] items: Vec<DonutItem>,
    /// Text shown in the donut center (defaults to the sum of all values).
    #[prop(optional, into)] center_label: Option<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let total: u32 = items.iter().map(|i| i.value).sum();
    let center_text = center_label.unwrap_or_else(|| total.to_string());

    let pi = std::f64::consts::PI;
    let mut angle = -pi / 2.0;

    let slices: Vec<_> = items
        .iter()
        .map(|item| {
            let frac = if total == 0 {
                1.0 / items.len() as f64
            } else {
                item.value as f64 / total as f64
            };
            let sweep = frac * 2.0 * pi;
            let end = angle + sweep.max(0.01); // avoid degenerate zero-sweep paths

            let (ox1, oy1, ox2, oy2, large) = arc_path(CX, CY, R, angle, end);
            let (ix1, iy1, ix2, iy2, _) = arc_path(CX, CY, R_INNER, angle, end);

            // Annular sector: outer arc CW, inner arc CCW
            let d = format!(
                "M{ox1:.2},{oy1:.2} A{R:.1},{R:.1} 0 {large},1 {ox2:.2},{oy2:.2} \
                 L{ix2:.2},{iy2:.2} A{R_INNER:.1},{R_INNER:.1} 0 {large},0 {ix1:.2},{iy1:.2} Z"
            );
            let fill = item.color.clone();
            angle = end;
            (d, fill)
        })
        .collect();

    let legend_items: Vec<_> = items
        .iter()
        .map(|item| {
            let color = item.color.clone();
            let label = item.label.clone();
            let value = item.value;
            view! {
                <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
                    <span
                        class="inline-block w-2 h-2 rounded-full shrink-0"
                        style=format!("background-color:{}", color)
                    />
                    <span class="truncate">{label}</span>
                    <span class="ms-auto tabular-nums font-semibold text-foreground">{value}</span>
                </div>
            }
        })
        .collect();

    let svg_slices: Vec<_> = slices
        .into_iter()
        .map(|(d, fill)| {
            view! {
                <path d=d fill=fill stroke="hsl(var(--card))" stroke-width="1.5" />
            }
        })
        .collect();

    let wrapper = format!("flex flex-col gap-3 {}", class);

    view! {
        <div class=wrapper>
            <svg viewBox="0 0 120 120" class="w-full max-w-[120px] mx-auto h-auto" aria-hidden="true">
                {svg_slices}
                // Center label
                <text
                    x=format!("{CX:.0}") y=format!("{:.0}", CY - 6.0)
                    text-anchor="middle"
                    dominant-baseline="middle"
                    font-size="18"
                    font-weight="700"
                    fill="currentColor"
                    class="fill-foreground"
                >{center_text}</text>
                <text
                    x=format!("{CX:.0}") y=format!("{:.0}", CY + 10.0)
                    text-anchor="middle"
                    dominant-baseline="middle"
                    font-size="8"
                    fill="#9ca3af"
                >{"Total"}</text>
            </svg>
            <div class="flex flex-col gap-1.5">
                {legend_items}
            </div>
        </div>
    }
}
