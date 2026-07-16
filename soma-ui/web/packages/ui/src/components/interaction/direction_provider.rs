use leptos::prelude::*;

/// Wraps children in a `<div dir="...">` to set LTR/RTL direction for a subtree.
#[component]
pub fn DirectionProvider(
    /// "ltr" or "rtl"
    #[prop(into, default = String::from("ltr"))]
    dir: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div dir=dir>
            {children()}
        </div>
    }
}
