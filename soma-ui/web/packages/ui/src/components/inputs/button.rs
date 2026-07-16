use crate::components::shared::{disabled_cls, CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Destructive,
    Secondary,
    Link,
    Contrast,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    Icon,
}

#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] pill: bool,
    #[prop(optional)] leading_icon: Option<icondata::Icon>,
    #[prop(optional)] trailing_icon: Option<icondata::Icon>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90 shadow-elev-sm hover:shadow-elev",
        ButtonVariant::Outline => "border border-input bg-transparent hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90 shadow-elev-sm hover:shadow-elev",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-elev-sm hover:shadow-elev",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        ButtonVariant::Contrast => "bg-contrast text-contrast-foreground hover:bg-contrast/90 shadow-elev-sm hover:shadow-elev",
    };

    let size_class = match size {
        ButtonSize::Sm => "h-8 px-3 text-xs",
        ButtonSize::Md => "h-10 px-4 text-sm",
        ButtonSize::Lg => "h-12 px-6 text-base",
        ButtonSize::Icon => "h-10 w-10",
    };

    let shape_class = if pill { "rounded-full" } else { "rounded-md" };
    let disabled_class = disabled_cls(disabled);

    let combined = format!(
        "inline-flex items-center justify-center whitespace-nowrap {} font-medium {} {} {} {} {} {}",
        shape_class, CONTROL_MOTION, PRESSABLE, FOCUS_RING, variant_class, size_class, disabled_class
    );
    let combined = if class.is_empty() {
        combined
    } else {
        format!("{} {}", combined, class)
    };

    view! {
        <button class=combined disabled=disabled aria-label=aria_label>
            {leading_icon.map(|ic| view! { <Icon icon=Signal::derive(move || ic) attr:class="w-4 h-4 shrink-0 -ms-0.5" /> })}
            {children()}
            {trailing_icon.map(|ic| view! { <Icon icon=Signal::derive(move || ic) attr:class="w-4 h-4 shrink-0 -me-0.5" /> })}
        </button>
    }
}
