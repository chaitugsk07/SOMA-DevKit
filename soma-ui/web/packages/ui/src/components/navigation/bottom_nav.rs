use crate::components::shared::CONTROL_MOTION;
use leptos::prelude::*;

#[component]
pub fn BottomNav(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "flex items-center justify-around border-t border-border bg-card {}",
        class
    );
    view! {
        <nav class=combined>{children()}</nav>
    }
}

#[component]
pub fn BottomNavItem(
    #[prop(into)] label: String,
    #[prop(into, default = Signal::derive(|| false))] active: Signal<bool>,
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let base = format!(
        "flex flex-col items-center gap-0.5 px-4 py-2 text-xs font-medium {}",
        CONTROL_MOTION
    );

    if let Some(href) = href {
        view! {
            <a
                href=href
                class=move || if active.get() {
                    format!("{} text-primary", base)
                } else {
                    format!("{} text-muted-foreground hover:text-foreground", base)
                }
            >
                {children()}
                <span>{label.clone()}</span>
            </a>
        }
        .into_any()
    } else {
        view! {
            <button
                class=move || if active.get() {
                    format!("{} text-primary", base)
                } else {
                    format!("{} text-muted-foreground hover:text-foreground", base)
                }
                on:click=move |_| { if let Some(f) = on_click { f.run(()); } }
            >
                {children()}
                <span>{label.clone()}</span>
            </button>
        }
        .into_any()
    }
}
