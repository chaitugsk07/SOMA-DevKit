use leptos::prelude::*;

const CX: f64 = 60.0;
const CY: f64 = 58.0;
const RADIUS: f64 = 44.0;
const STROKE: f64 = 10.0;

/// Gauge spans 270°: from 135° (bottom-left) to 45° (bottom-right), clockwise.
const START_DEG: f64 = 135.0;
const SWEEP_DEG: f64 = 270.0;

fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

fn arc_point(angle_deg: f64) -> (f64, f64) {
    let a = deg_to_rad(angle_deg);
    (CX + RADIUS * a.cos(), CY + RADIUS * a.sin())
}

/// SVG path for the full 270° gauge arc (used for both background track and value arc).
fn full_arc_d() -> String {
    let (sx, sy) = arc_point(START_DEG);
    let (ex, ey) = arc_point(START_DEG + SWEEP_DEG); // 405° = 45°
    // large_arc=1 (270° > 180°), sweep=1 (clockwise in SVG)
    format!("M{sx:.2},{sy:.2} A{RADIUS:.1},{RADIUS:.1} 0 1,1 {ex:.2},{ey:.2}")
}

fn gauge_color(value: u32) -> &'static str {
    if value < 35 {
        "#3fb950" // low
    } else if value < 60 {
        "#f5c518" // medium
    } else if value < 80 {
        "#f5820a" // high
    } else {
        "#e5484d" // critical
    }
}

/// Arc gauge (270°) showing a 0–100 risk score.
///
/// Color by band: <35 low (green), <60 medium (yellow), <80 high (orange), ≥80 critical (red).
/// The filled arc grows clockwise from the bottom-left; a muted track sits behind it.
#[component]
pub fn RiskGauge(
    /// Risk score, 0–100 (clamped defensively).
    value: u32,
    /// Optional label shown below the numeric value.
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let value = value.min(100);
    // Total arc length for the 270° sweep
    let arc_length = SWEEP_DEG / 360.0 * 2.0 * std::f64::consts::PI * RADIUS;
    let dash = value as f64 / 100.0 * arc_length;

    let color = gauge_color(value);
    let d = full_arc_d();
    // gap larger than arc_length prevents the dash pattern from repeating
    let dash_array = format!("{dash:.2} {arc_length:.2}");
    let value_str = value.to_string();
    let wrapper = format!("inline-flex flex-col items-center {class}");

    view! {
        <div class=wrapper>
            <svg
                viewBox="0 0 120 100"
                class="w-full max-w-[120px] h-auto"
                aria-label=format!("Risk score {value}")
            >
                // Muted background track
                <path
                    d=d.clone()
                    fill="none"
                    stroke="hsl(var(--border))"
                    stroke-width=STROKE.to_string()
                    stroke-linecap="round"
                />
                // Colored value arc
                <path
                    d=d
                    fill="none"
                    stroke=color
                    stroke-width=STROKE.to_string()
                    stroke-linecap="round"
                    stroke-dasharray=dash_array
                />
                // Centered numeric value
                <text
                    x="60" y="55"
                    text-anchor="middle"
                    dominant-baseline="middle"
                    font-size="22"
                    font-weight="700"
                    class="fill-foreground"
                >{value_str}</text>
                // Optional label beneath value
                {label.map(|l| view! {
                    <text
                        x="60" y="72"
                        text-anchor="middle"
                        dominant-baseline="middle"
                        font-size="9"
                        class="fill-muted-foreground"
                    >{l}</text>
                })}
            </svg>
        </div>
    }
}
