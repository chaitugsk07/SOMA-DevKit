use leptos::prelude::*;

#[component]
pub fn InlineCode(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "font-mono text-xs bg-muted/60 px-1 py-0.5 rounded border border-border {}",
        class
    );
    view! {
        <code class=combined>{children()}</code>
    }
}
