use leptos::prelude::*;

#[component]
pub fn Skeleton(#[prop(default = String::new())] class: String) -> impl IntoView {
    let combined = format!("animate-pulse rounded-md bg-muted {}", class);
    view! { <div class=combined></div> }
}
