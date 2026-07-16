use crate::components::shared::CONTROL_MOTION;
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChipVariant {
    #[default]
    Default,
    Secondary,
    Outline,
}

#[component]
pub fn Chip(
    #[prop(default = ChipVariant::Default)] variant: ChipVariant,
    #[prop(default = false)] removable: bool,
    #[prop(optional)] on_remove: Option<Callback<()>>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ChipVariant::Default => {
            "bg-primary text-primary-foreground border-transparent shadow-elev-sm"
        }
        ChipVariant::Secondary => {
            "bg-secondary text-secondary-foreground border-transparent shadow-elev-sm"
        }
        ChipVariant::Outline => "bg-transparent text-foreground border-border",
    };
    let combined = format!(
        "inline-flex items-center gap-1 rounded-full border px-2.5 py-0.5 text-xs {} {}",
        variant_class, class
    );
    view! {
        <span class=combined>
            {children()}
            {if removable {
                view! {
                    <button
                        class=format!("ms-0.5 rounded-full hover:opacity-70 {} focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring", CONTROL_MOTION)
                        on:click=move |_| { if let Some(cb) = on_remove { cb.run(()); } }
                    >
                        <Icon icon=Signal::derive(|| icondata::LuX) width="10" height="10" />
                    </button>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </span>
    }
}
