use leptos::prelude::*;

#[component]
pub fn SaveBar(
    #[prop(into, optional)] message: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "sticky bottom-0 flex items-center justify-between gap-3 border-t border-border bg-background/80 backdrop-blur-md p-3.5 {}",
        class
    );
    view! {
        <div class=combined>
            <div>
                {message.map(|m| view! { <strong class="text-sm">{m}</strong> })}
                {description.map(|d| view! { <span class="text-xs text-muted-foreground">{d}</span> })}
            </div>
            <div>{children()}</div>
        </div>
    }
}
