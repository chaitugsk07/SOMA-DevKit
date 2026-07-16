use leptos::prelude::*;

// ponytail: pure CSS hover — group/group-hover only. Zero JS, zero signals.

#[component]
pub fn Tooltip(children: Children) -> impl IntoView {
    view! {
        <div class="group relative inline-block">
            {children()}
        </div>
    }
}

#[component]
pub fn TooltipTrigger(children: Children) -> impl IntoView {
    view! {
        <>{children()}</>
    }
}

#[component]
pub fn TooltipContent(children: Children) -> impl IntoView {
    view! {
        <div class="absolute z-50 bottom-full mb-2 left-1/2 -translate-x-1/2 whitespace-nowrap rounded-md bg-card px-2 py-1 text-xs text-foreground shadow-elev opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 pointer-events-none transition">
            {children()}
        </div>
    }
}
