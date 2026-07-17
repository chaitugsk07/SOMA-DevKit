use crate::{Status, StatusKind};
use leptos::prelude::*;

/// A filter toolbar row: filter controls on the left, optional live status on the right.
#[component]
pub fn FilterRail(
    #[prop(optional, into)] live_label: Option<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "grid grid-cols-[1fr_auto] items-end gap-3 border-b border-border p-3.5 {}",
        class
    );
    view! {
        <div class=combined>
            <div class="flex flex-wrap items-end gap-3">{children()}</div>
            {live_label.map(|l| view! {
                <Status kind=StatusKind::Online label=l />
            })}
        </div>
    }
}
