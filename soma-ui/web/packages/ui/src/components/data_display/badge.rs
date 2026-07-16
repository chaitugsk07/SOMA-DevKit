use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Success,
}

#[component]
pub fn Badge(
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        BadgeVariant::Default => {
            "bg-primary text-primary-foreground border-transparent shadow-elev-sm"
        }
        BadgeVariant::Secondary => {
            "bg-secondary text-secondary-foreground border-transparent shadow-elev-sm"
        }
        BadgeVariant::Destructive => {
            "bg-destructive text-destructive-foreground border-transparent shadow-elev-sm"
        }
        BadgeVariant::Outline => "text-foreground border border-border",
        BadgeVariant::Success => {
            "bg-success text-success-foreground border-transparent shadow-elev-sm"
        }
    };

    let combined = format!(
        "inline-flex items-center whitespace-nowrap rounded border px-2.5 py-0.5 text-xs font-semibold transition-colors {} {}",
        variant_class, class
    );

    view! {
        <span class=combined>{children()}</span>
    }
}
