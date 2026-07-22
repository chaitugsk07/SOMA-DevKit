use leptos::prelude::*;

// ponytail: pure CSS hover via group/group-hover. No JS, no signals.

#[component]
pub fn HoverCard(children: Children) -> impl IntoView {
    view! {
        <div class="group relative inline-block">
            {children()}
        </div>
    }
}

#[component]
pub fn HoverCardTrigger(children: Children) -> impl IntoView {
    view! {
        <>{children()}</>
    }
}

#[component]
pub fn HoverCardContent(children: Children) -> impl IntoView {
    view! {
        <div class="absolute z-50 top-full mt-2 w-64 rounded-md border border-border bg-card text-card-foreground p-4 shadow-elev-md opacity-0 invisible group-hover:opacity-100 group-hover:visible transition">
            {children()}
        </div>
    }
}
