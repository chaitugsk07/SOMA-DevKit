use leptos::prelude::*;

#[component]
pub fn Label(
    #[prop(optional)] for_id: Option<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "text-sm font-medium leading-none text-foreground peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {}",
        class
    );
    view! {
        <label for=for_id class=combined>{children()}</label>
    }
}
