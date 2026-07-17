use leptos::prelude::*;

#[component]
pub fn MasterDetail(
    #[prop(default = String::new())] class: String,
    list: AnyView,
    detail: AnyView,
) -> impl IntoView {
    let combined = format!(
        "grid gap-3.5 min-h-[590px] grid-cols-1 md:grid-cols-[minmax(280px,.72fr)_minmax(0,1.28fr)] {}",
        class
    );
    view! {
        <div class=combined>
            {list}
            {detail}
        </div>
    }
}
