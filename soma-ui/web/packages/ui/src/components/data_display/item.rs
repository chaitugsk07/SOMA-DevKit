use crate::components::shared::CONTROL_MOTION;
use leptos::prelude::*;

// ponytail: minimal composable row — consumers put leading icon / content / trailing action inside.
// Upgrade path: add explicit leading/content/trailing slot props if layout needs diverge.
#[component]
pub fn Item(#[prop(default = String::new())] class: String, children: Children) -> impl IntoView {
    let combined = format!(
        "flex items-center gap-3 rounded-md border border-border p-3 {} hover:shadow-elev hover:border-ring/40 {}",
        CONTROL_MOTION, class
    );
    view! { <div class=combined>{children()}</div> }
}
