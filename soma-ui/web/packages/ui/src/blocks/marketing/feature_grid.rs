use crate::components::data_display::card::{
    Card, CardContent, CardDescription, CardHeader, CardTitle,
};
use leptos::prelude::*;

#[component]
pub fn FeatureGrid(
    #[prop(default = 3usize)] cols: usize,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    // ponytail: grid cols class computed at runtime from usize. Tailwind JIT needs
    // full class strings in source for purging. Ceiling: only cols 1-4 are safe;
    // other values fall back to 3. Upgrade path: add more arms or use inline style.
    let cols_class = match cols {
        1 => "grid-cols-1",
        2 => "md:grid-cols-2",
        3 => "md:grid-cols-3",
        4 => "md:grid-cols-4",
        _ => "md:grid-cols-3",
    };
    let combined = format!("grid grid-cols-1 {} gap-6 {}", cols_class, class);
    view! { <div class=combined>{children()}</div> }
}

#[component]
pub fn FeatureCard(
    #[prop(into, optional)] icon: Option<String>,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <Card class=class>
            <CardHeader>
                {icon.map(|i| view! {
                    <div class="text-2xl mb-2">{i}</div>
                })}
                <CardTitle>{title}</CardTitle>
                <CardDescription>{description}</CardDescription>
            </CardHeader>
            {children.map(|c| view! {
                <CardContent>{c()}</CardContent>
            })}
        </Card>
    }
}
