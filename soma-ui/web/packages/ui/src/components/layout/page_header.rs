use crate::Eyebrow;
use leptos::prelude::*;

#[component]
pub fn PageHeader(
    #[prop(into)] title: String,
    #[prop(default = None)] subtitle: Option<String>,
    #[prop(into, optional)] eyebrow: Option<String>,
    #[prop(optional)] back_slot: Option<AnyView>,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!(
        "flex items-start justify-between gap-4 mb-6 pb-6 border-b border-border {}",
        class
    );
    view! {
        <div class=combined>
            <div class="space-y-1">
                {back_slot}
                {eyebrow.map(|e| view! { <Eyebrow>{e}</Eyebrow> })}
                <h1 class="font-heading text-3xl font-bold text-foreground">{title}</h1>
                {subtitle.map(|s| view! { <p class="text-sm text-muted-foreground">{s}</p> })}
            </div>
            {children.map(|c| view! { <div class="flex items-center gap-2 shrink-0">{c()}</div> })}
        </div>
    }
}
