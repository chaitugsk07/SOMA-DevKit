use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CalloutVariant {
    #[default]
    Default,
    Info,
    Warning,
}

#[component]
pub fn Callout(
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(optional)] title: Option<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let (border_class, title_class) = match variant {
        CalloutVariant::Default => ("border-border", "text-foreground"),
        CalloutVariant::Info => ("border-blue-500", "text-blue-500"),
        CalloutVariant::Warning => ("border-yellow-500", "text-yellow-500"),
    };
    let combined = format!(
        "border-l-4 rounded-md p-4 bg-muted/40 shadow-elev-sm {} {}",
        border_class, class
    );
    view! {
        <div class=combined>
            {title.map(|t| view! { <p class=format!("font-medium mb-1 {}", title_class)>{t}</p> })}
            {children()}
        </div>
    }
}
