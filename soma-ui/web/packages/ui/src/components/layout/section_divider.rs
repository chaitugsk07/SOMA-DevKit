use leptos::prelude::*;

#[component]
pub fn SectionDivider(
    #[prop(into, optional)] label: Option<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!(
        "flex items-center gap-2 border-t border-border pt-3 {}",
        class
    );
    view! {
        <div class=combined>
            {label.map(|l| view! {
                <span class="text-[9px] uppercase tracking-wider text-muted-foreground">{l}</span>
            })}
        </div>
    }
}
