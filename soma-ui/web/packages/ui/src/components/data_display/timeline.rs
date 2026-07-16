use crate::components::data_display::badge::{Badge, BadgeVariant};
use leptos::prelude::*;

#[component]
pub fn Timeline(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("flex flex-col {}", class);
    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn TimelineItem(
    #[prop(into)] marker: String,
    #[prop(into)] title: String,
    #[prop(into, optional)] status: Option<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let badge_variant = match status.as_deref() {
        Some("applied") | Some("success") => BadgeVariant::Success,
        Some("pending") => BadgeVariant::Secondary,
        Some("error") => BadgeVariant::Destructive,
        _ => BadgeVariant::Default,
    };

    // [&:last-child>div:first-child>div:last-child]:opacity-0 hides the connector line on last item
    let outer = format!(
        "flex gap-4 [&:last-child>div:first-child>div:last-child]:opacity-0 {}",
        class
    );

    view! {
        <div class=outer>
            <div class="flex flex-col items-center">
                <Badge variant=badge_variant class="h-7 w-7 flex items-center justify-center rounded-full p-0 shrink-0".to_string()>
                    {marker}
                </Badge>
                <div class="flex-1 w-px bg-border mt-1" />
            </div>
            <div class="pb-6 flex-1 min-w-0">
                <p class="text-sm font-medium text-foreground leading-none mb-1">{title}</p>
                <div class="text-sm text-muted-foreground">{children()}</div>
            </div>
        </div>
    }
}
