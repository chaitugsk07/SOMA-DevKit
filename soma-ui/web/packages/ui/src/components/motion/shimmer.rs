use leptos::prelude::*;

#[component]
pub fn Shimmer(#[prop(default = String::new())] class: String) -> impl IntoView {
    let combined = format!(
        "animate-shimmer rounded-md bg-gradient-to-r from-muted via-muted/40 to-muted bg-[length:200%_100%] {}",
        class
    );
    view! {
        <div class=combined />
    }
}
