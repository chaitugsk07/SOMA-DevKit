use crate::charts::{sparkline::Sparkline, ChartPoint};
use leptos::prelude::*;

/// A single-value dashboard tile with optional delta and embedded Sparkline.
///
/// Layout (top→bottom):
///   - `label`  — muted, small text
///   - `value`  — large, tabular-nums font
///   - `delta`  — optional percent change; ▲ green for positive, ▼ red for negative
///   - Sparkline — rendered when `spark` is non-empty
#[component]
pub fn StatTile(
    label: String,
    value: String,
    #[prop(optional)] delta: Option<f64>,
    #[prop(default = vec![])] spark: Vec<ChartPoint>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let container = format!(
        "rounded-lg border border-border bg-card p-4 flex flex-col gap-1 {}",
        class
    );

    let delta_view = delta.map(|d| {
        let (arrow, color) = if d >= 0.0 {
            ("\u{25b2}", "text-emerald-500") // ▲
        } else {
            ("\u{25bc}", "text-red-500") // ▼
        };
        let text = format!("{} {:.1}%", arrow, d.abs());
        view! {
            <span class=format!("text-xs font-medium {}", color)>{text}</span>
        }
    });

    let has_spark = !spark.is_empty();

    view! {
        <div class=container>
            <span class="text-xs text-muted-foreground font-medium truncate">{label}</span>
            <span class="text-2xl font-semibold tabular-nums text-foreground leading-tight">
                {value}
            </span>
            {delta_view}
            {has_spark.then(|| view! {
                <div class="mt-2 h-8">
                    <Sparkline data=spark />
                </div>
            })}
        </div>
    }
}
