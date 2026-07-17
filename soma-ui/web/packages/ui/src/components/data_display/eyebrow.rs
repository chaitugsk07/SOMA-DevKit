use leptos::prelude::*;

#[component]
pub fn Eyebrow(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "text-[10px] font-bold uppercase tracking-[0.16em] text-[color:var(--soma-brand,hsl(var(--primary)))] {}",
        class
    );
    view! {
        <p class=combined>{children()}</p>
    }
}
