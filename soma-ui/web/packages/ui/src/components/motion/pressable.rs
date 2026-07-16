use leptos::prelude::*;

#[component]
pub fn Pressable(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "inline-block active:scale-95 transition-transform duration-100 {}",
        class
    );
    view! {
        <div class=combined>{children()}</div>
    }
}
