use leptos::prelude::*;

/// ponytail: controlled-only (caller owns the signal). Uncontrolled variant = caller creates `RwSignal::new(false)`.
#[component]
pub fn Collapsible(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = String::new())] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div class=class>
            <Show when=move || open.get()>
                <div class="animate-slide-down">
                    {children()}
                </div>
            </Show>
        </div>
    }
}
