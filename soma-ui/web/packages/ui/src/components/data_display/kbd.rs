use leptos::prelude::*;

#[component]
pub fn Kbd(#[prop(default = String::new())] class: String, children: Children) -> impl IntoView {
    let combined = format!(
        "inline-flex items-center rounded border border-border bg-muted px-1.5 py-0.5 text-xs font-medium text-muted-foreground shadow-elev-sm {}",
        class
    );
    view! { <kbd class=combined>{children()}</kbd> }
}
