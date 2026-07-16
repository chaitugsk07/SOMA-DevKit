use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CardPadding {
    #[default]
    None,
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Card(
    #[prop(default = CardPadding::None)] padding: CardPadding,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "bg-card border border-border rounded-xl text-card-foreground shadow-elev-sm {}",
        class
    );
    view! {
        <div class=combined>
            {match padding {
                CardPadding::None => children().into_any(),
                CardPadding::Sm => view! { <div class="p-4">{children()}</div> }.into_any(),
                CardPadding::Md => view! { <div class="p-5">{children()}</div> }.into_any(),
                CardPadding::Lg => view! { <div class="p-6">{children()}</div> }.into_any(),
            }}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("flex flex-col space-y-1.5 p-6 {}", class);
    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn CardTitle(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "font-heading text-lg font-semibold leading-none tracking-tight {}",
        class
    );
    view! { <h3 class=combined>{children()}</h3> }
}

#[component]
pub fn CardDescription(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("text-sm text-muted-foreground {}", class);
    view! { <p class=combined>{children()}</p> }
}

#[component]
pub fn CardContent(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("p-6 pt-0 {}", class);
    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn CardFooter(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("flex items-center p-6 pt-0 {}", class);
    view! { <div class=combined>{children()}</div> }
}
