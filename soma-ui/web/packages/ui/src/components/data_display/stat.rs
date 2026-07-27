use crate::components::data_display::card::Card;
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatTrend {
    Up,
    Down,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum StatSize {
    #[default]
    Md,
    Sm,
}

#[component]
pub fn Stat(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(default = None)] delta: Option<String>,
    #[prop(default = None)] trend: Option<StatTrend>,
    #[prop(optional)] icon: Option<icondata::Icon>,
    #[prop(default = StatSize::Md)] size: StatSize,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let delta_view = delta.map(|d| {
        let (trend_class, prefix) = match trend {
            Some(StatTrend::Up) => ("bg-success/10 text-success", "\u{2191} "),
            Some(StatTrend::Down) => ("bg-destructive/10 text-destructive", "\u{2193} "),
            _ => ("bg-muted text-muted-foreground", ""),
        };
        let text = format!("{}{}", prefix, d);
        let chip_class = format!(
            "inline-flex items-center gap-1 text-xs font-medium mt-2 px-2 py-0.5 rounded-full {}",
            trend_class
        );
        view! { <span class=chip_class>{text}</span> }
    });

    let value_class = match size {
        StatSize::Md => "text-2xl font-semibold tabular-nums text-foreground mt-1",
        StatSize::Sm => "text-lg font-semibold tabular-nums text-foreground mt-1",
    };

    let icon_view = icon.map(|ic| {
        view! {
            <div class="p-2 bg-accent rounded-md">
                <Icon icon=Signal::derive(move || ic) attr:class="w-5 h-5 text-muted-foreground" />
            </div>
        }
    });

    view! {
        <Card class=class>
            <div class="p-4">
                <div class="flex items-start justify-between">
                    <div>
                        <p class="text-xs font-medium uppercase tracking-[0.08em] text-muted-foreground">{label}</p>
                        <p class=value_class>{value}</p>
                        {delta_view}
                    </div>
                    {icon_view}
                </div>
            </div>
        </Card>
    }
}
