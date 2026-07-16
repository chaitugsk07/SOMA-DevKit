use leptos::prelude::*;

#[component]
pub fn AspectRatio(
    #[prop(default = 16.0 / 9.0)] ratio: f64,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("w-full overflow-hidden {}", class);
    view! {
        <div class=combined style=format!("aspect-ratio:{}", ratio)>
            {children()}
        </div>
    }
}
