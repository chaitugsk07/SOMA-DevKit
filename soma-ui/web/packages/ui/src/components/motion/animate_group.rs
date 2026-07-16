use leptos::prelude::*;

// ponytail: uniform animate-slide-up on all children via CSS selector.
// Ceiling: all children share the same timing; no per-child stagger.
// Upgrade: set animation-delay on each child via a wrapper loop.
#[component]
pub fn AnimateGroup(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("[&>*]:animate-slide-up {}", class);
    view! {
        <div class=combined>{children()}</div>
    }
}
