use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Success,
    Warning,
    Info,
}

#[component]
pub fn Alert(
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        AlertVariant::Default => "border-l-4 border-primary bg-primary/5 text-foreground",
        AlertVariant::Destructive => {
            "border-l-4 border-destructive/50 bg-destructive/5 text-destructive"
        }
        AlertVariant::Success => "border-l-4 border-success bg-success/5 text-success",
        AlertVariant::Warning => "border-l-4 border-warning bg-warning/5 text-warning",
        AlertVariant::Info => "border-l-4 border-info bg-info/5 text-info",
    };
    let combined = format!(
        "relative w-full rounded-lg border p-4 shadow-elev-sm {} {}",
        variant_class, class
    );
    view! { <div role="alert" class=combined>{children()}</div> }
}

#[component]
pub fn AlertTitle(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("mb-1 font-medium leading-none tracking-tight {}", class);
    view! { <h5 class=combined>{children()}</h5> }
}

#[component]
pub fn AlertDescription(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("text-sm text-muted-foreground {}", class);
    view! { <div class=combined>{children()}</div> }
}
