use leptos::prelude::*;

#[component]
pub fn Marquee(
    #[prop(default = false)] reverse: bool,
    #[prop(default = String::new())] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let track_class = if reverse {
        "flex w-max animate-marquee [animation-direction:reverse]"
    } else {
        "flex w-max animate-marquee"
    };
    view! {
        <div class=format!("overflow-hidden {}", class)>
            <div class=track_class>
                {children()}
                {children()}
            </div>
        </div>
    }
}
