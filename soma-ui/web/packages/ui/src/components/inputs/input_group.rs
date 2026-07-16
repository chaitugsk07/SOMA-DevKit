use crate::components::shared::CONTROL_MOTION;
use leptos::prelude::*;

/// Wraps an `Input` (and optional addons) with a shared border and focus ring.
#[component]
pub fn InputGroup(children: Children) -> impl IntoView {
    let combined = format!(
        "flex items-center rounded-md border border-input bg-background shadow-elev-sm hover:border-ring/60 focus-within:ring-2 focus-within:ring-ring overflow-hidden {}",
        CONTROL_MOTION
    );
    view! {
        <div class=combined>
            {children()}
        </div>
    }
}

/// A non-interactive prefix or suffix label inside an `InputGroup`.
#[component]
pub fn InputGroupAddon(children: Children) -> impl IntoView {
    view! {
        <div class="flex items-center px-3 text-sm text-muted-foreground bg-muted border-e border-input shrink-0">
            {children()}
        </div>
    }
}
