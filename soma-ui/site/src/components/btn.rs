use leptos::prelude::*;

/// Button style variant.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum BtnVariant {
    /// Filled amber background — primary action
    #[default]
    Solid,
    /// Hairline amber border, transparent background
    Outline,
    /// No border, no background — inline link-style
    Ghost,
}

/// An anchor styled as a button. Always renders as `<a>` (link, no form submission).
/// Use `Solid` for primary CTAs, `Outline` for secondary, `Ghost` for tertiary.
#[component]
pub fn Btn(
    #[prop(into)] href: String,
    #[prop(default = BtnVariant::Solid)] variant: BtnVariant,
    #[prop(optional, into)] aria_label: String,
    children: Children,
) -> impl IntoView {
    let cls = match variant {
        BtnVariant::Solid   => "btn btn--solid",
        BtnVariant::Outline => "btn btn--outline",
        BtnVariant::Ghost   => "btn btn--ghost",
    };
    view! {
        <a
            href=href
            class=cls
            aria-label=if aria_label.is_empty() { None } else { Some(aria_label) }
        >
            {children()}
        </a>
    }
}
