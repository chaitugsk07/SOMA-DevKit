use leptos::prelude::*;

#[component]
pub fn ScrollArea(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "overflow-auto [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-border [&::-webkit-scrollbar-track]:bg-transparent {}",
        class
    );
    view! {
        <div class=combined>{children()}</div>
    }
}
