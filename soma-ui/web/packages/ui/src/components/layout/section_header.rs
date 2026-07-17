use crate::Eyebrow;
use leptos::prelude::*;

#[component]
pub fn SectionHeader(
    #[prop(into)] title: String,
    #[prop(into, optional)] eyebrow: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let combined = format!("flex items-end justify-between gap-3 {}", class);
    view! {
        <div class=combined>
            <div>
                {eyebrow.map(|e| view! { <Eyebrow>{e}</Eyebrow> })}
                <h2 class="text-base font-semibold text-foreground">{title}</h2>
            </div>
            {children.map(|c| c())}
        </div>
    }
}
