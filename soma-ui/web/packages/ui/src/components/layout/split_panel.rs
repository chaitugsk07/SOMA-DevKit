use leptos::prelude::*;

#[component]
pub fn SplitPanel(
    #[prop(default = String::from("330px"))] left_width: String,
    #[prop(default = String::new())] class: String,
    left: AnyView,
    right: AnyView,
) -> impl IntoView {
    let combined = format!("grid overflow-hidden border border-border rounded-[14px_3px_14px_3px] {}", class);
    let style = format!("grid-template-columns: {} 1fr", left_width);
    view! {
        <div class=combined style=style>
            <div class="border-r border-border bg-muted/30">{left}</div>
            <div class="min-w-0">{right}</div>
        </div>
    }
}
