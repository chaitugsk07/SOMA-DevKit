use leptos::prelude::*;

/// Thin callable wrapper for the `Empty` icon slot.
/// Accepts any `|| view! {…}` closure via `#[prop(into)]`.
///
/// ```rust,ignore
/// <Empty icon=|| view! { <svg>…</svg> } title="…" />
/// ```
pub struct EmptyIcon(ChildrenFn);

impl EmptyIcon {
    fn render(&self) -> AnyView {
        (self.0)()
    }
}

impl<F: Fn() -> AnyView + Send + Sync + 'static> From<F> for EmptyIcon {
    fn from(f: F) -> Self {
        EmptyIcon(std::sync::Arc::new(f))
    }
}

#[component]
pub fn Empty(
    #[prop(default = String::new())] title: String,
    #[prop(optional)] description: Option<String>,
    /// Optional icon rendered above the title. Pass a closure:
    /// `icon=|| view! { <svg>…</svg> }.into_any()`.
    #[prop(optional, into)] icon: Option<EmptyIcon>,
    /// Optional CTA content (e.g. a Button) rendered below the description.
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!(
        "flex flex-col items-center justify-center text-center p-8 gap-3 {}",
        class
    );
    view! {
        <div class=combined>
            // icon → title → description → CTA
            {icon.map(|ic| ic.render())}
            {if !title.is_empty() {
                view! { <p class="font-medium text-foreground">{title}</p> }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
            {description.map(|d| view! { <p class="text-sm text-muted-foreground">{d}</p> })}
            {children.map(|c| c())}
        </div>
    }
}
