use leptos::prelude::*;

pub struct PropertyRow {
    pub label: String,
    pub value: AnyView,
}

#[component]
pub fn PropertyList(
    items: Vec<PropertyRow>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div class=class style="display:grid">
            {items.into_iter().map(|row| view! {
                <div class="border-b border-border py-1.5">
                    <span class="block text-[10px] uppercase tracking-wider text-muted-foreground">
                        {row.label}
                    </span>
                    <div class="text-xs text-foreground">{row.value}</div>
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}
