use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AnimationType {
    #[default]
    FadeIn,
    SlideUp,
    SlideDown,
    SlideLeft,
    SlideRight,
    ScaleIn,
    BounceIn,
    PulseSoft,
}

#[component]
pub fn Animate(
    #[prop(default = AnimationType::FadeIn)] animation: AnimationType,
    #[prop(default = String::new())] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let anim_class = match animation {
        AnimationType::FadeIn => "animate-fade-in",
        AnimationType::SlideUp => "animate-slide-up",
        AnimationType::SlideDown => "animate-slide-down",
        AnimationType::SlideLeft => "animate-slide-left",
        AnimationType::SlideRight => "animate-slide-right",
        AnimationType::ScaleIn => "animate-scale-in",
        AnimationType::BounceIn => "animate-bounce-in",
        AnimationType::PulseSoft => "animate-pulse-soft",
    };

    let combined = format!("{} {}", anim_class, class);

    view! {
        <div class=combined>{children()}</div>
    }
}
