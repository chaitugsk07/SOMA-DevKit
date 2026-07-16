use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Spinner(
    #[prop(default = SpinnerSize::Md)] size: SpinnerSize,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let size_class = match size {
        SpinnerSize::Sm => "h-4 w-4",
        SpinnerSize::Md => "h-6 w-6",
        SpinnerSize::Lg => "h-8 w-8",
    };
    let combined = format!(
        "animate-spin rounded-full border-2 border-current border-t-transparent text-muted-foreground {} {}",
        size_class, class
    );
    view! { <div class=combined></div> }
}
