use leptos::prelude::*;

#[component]
pub fn Progress(
    #[prop(default = 0.0)] value: f64,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let track_class = format!(
        "h-2 w-full overflow-hidden rounded-full bg-secondary {}",
        class
    );
    let width = format!("width: {}%", value.clamp(0.0, 100.0));
    view! {
        <div class=track_class>
            <div class="h-full bg-primary transition-all" style=width></div>
        </div>
    }
}
