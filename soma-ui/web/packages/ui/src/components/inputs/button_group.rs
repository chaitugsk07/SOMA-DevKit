use leptos::prelude::*;

/// Pure layout wrapper that removes individual button rounding and merges borders
/// so grouped buttons appear as a single connected control.
#[component]
pub fn ButtonGroup(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "inline-flex [&>button]:rounded-none [&>button:first-child]:rounded-s-md [&>button:last-child]:rounded-e-md [&>button:not(:first-child)]:-ms-px {}",
        class
    );
    view! {
        <div class=combined>{children()}</div>
    }
}
