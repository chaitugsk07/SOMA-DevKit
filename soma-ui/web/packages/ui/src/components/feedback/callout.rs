use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CalloutVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Destructive,
}

#[component]
pub fn Callout(
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(optional)] title: Option<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let (border_class, title_class, background_class) = match variant {
        CalloutVariant::Default => ("border-border", "text-foreground", "bg-muted/40"),
        CalloutVariant::Info => ("border-info", "text-info", "bg-info/5"),
        CalloutVariant::Success => ("border-success", "text-success", "bg-success/5"),
        CalloutVariant::Warning => ("border-warning", "text-warning", "bg-warning/5"),
        CalloutVariant::Destructive => {
            ("border-destructive", "text-destructive", "bg-destructive/5")
        }
    };
    let combined = format!(
        "border-l-4 rounded-md p-4 shadow-elev-sm {} {} {}",
        border_class, background_class, class
    );
    view! {
        <div class=combined>
            {title.map(|t| view! { <p class=format!("font-medium mb-1 {}", title_class)>{t}</p> })}
            {children()}
        </div>
    }
}
