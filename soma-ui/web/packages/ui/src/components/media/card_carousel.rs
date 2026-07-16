use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;
use leptos::web_sys;

// ponytail: native CSS scroll-snap horizontal scroller.
// No JS index tracking needed — browser handles snapping natively.
// Ceiling: programmatic scroll-to-index, keyboard navigation = upgrade path
// (add NodeRef + scroll_by or scroll_into_view calls).

#[component]
pub fn CardCarousel(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let scroll_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let scroll_ref2 = scroll_ref;

    let scroll = move |delta: f64| {
        if let Some(el) = scroll_ref.get() {
            let el: &web_sys::Element = el.as_ref();
            el.scroll_by_with_x_and_y(delta, 0.0);
        }
    };
    let scroll2 = move |delta: f64| {
        if let Some(el) = scroll_ref2.get() {
            let el: &web_sys::Element = el.as_ref();
            el.scroll_by_with_x_and_y(delta, 0.0);
        }
    };

    let combined = format!("relative {}", class);

    view! {
        <div class=combined>
            // Prev button
            <button
                class=format!("absolute start-0 top-1/2 -translate-y-1/2 z-10 flex h-8 w-8 items-center justify-center rounded-full border border-border bg-background text-foreground hover:bg-accent shadow-elev-sm {} {} {}", CONTROL_MOTION, PRESSABLE, FOCUS_RING)
                on:click=move |_| scroll(-280.0)
                aria-label="Scroll left"
            >
                <Icon icon=Signal::derive(|| icondata::LuChevronLeft) width="16" height="16" />
            </button>

            // Scrollable track
            <div
                node_ref=scroll_ref
                class="flex gap-4 overflow-x-auto snap-x snap-mandatory scroll-smooth px-10 pb-2 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
            >
                {children()}
            </div>

            // Next button
            <button
                class=format!("absolute end-0 top-1/2 -translate-y-1/2 z-10 flex h-8 w-8 items-center justify-center rounded-full border border-border bg-background text-foreground hover:bg-accent shadow-elev-sm {} {} {}", CONTROL_MOTION, PRESSABLE, FOCUS_RING)
                on:click=move |_| scroll2(280.0)
                aria-label="Scroll right"
            >
                <Icon icon=Signal::derive(|| icondata::LuChevronRight) width="16" height="16" />
            </button>
        </div>
    }
}

/// A snapping card in a CardCarousel. Fixed width, shrink-0.
#[component]
pub fn CardCarouselItem(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "snap-start shrink-0 w-64 bg-card border border-border rounded-md overflow-hidden {}",
        class
    );
    view! {
        <div class=combined>
            {children()}
        </div>
    }
}
