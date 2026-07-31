use leptos::prelude::*;

/// Max-width wrapper. Centered, with horizontal padding.
#[component]
pub fn Container(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "container".to_string()
    } else {
        format!("container {class}")
    };
    view! { <div class=cls>{children()}</div> }
}

/// Semantic section with optional id and extra class.
#[component]
pub fn Section(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "section".to_string()
    } else {
        format!("section {class}")
    };
    view! {
        <section id=if id.is_empty() { None } else { Some(id) } class=cls>
            {children()}
        </section>
    }
}
