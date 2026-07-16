use leptos::prelude::*;

#[component]
pub fn Empty(
    #[prop(default = String::new())] title: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!(
        "flex flex-col items-center justify-center text-center p-8 gap-2 {}",
        class
    );
    view! {
        <div class=combined>
            {children.map(|c| c())}
            {if !title.is_empty() {
                view! { <p class="font-medium text-foreground">{title}</p> }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
            {description.map(|d| view! { <p class="text-sm text-muted-foreground">{d}</p> })}
        </div>
    }
}
