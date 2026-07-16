use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

// ponytail: one-per-view carousel with clamp navigation.
// Ceiling: multi-item per view, autoplay, touch swipe = upgrade path.

#[derive(Clone, Copy)]
struct CarouselCtx {
    index: RwSignal<usize>,
    count: StoredValue<usize>,
}

#[component]
pub fn Carousel(
    /// Total number of slides.
    count: usize,
    children: Children,
) -> impl IntoView {
    let ctx = CarouselCtx {
        index: RwSignal::new(0),
        count: StoredValue::new(count),
    };
    provide_context(ctx);

    view! {
        <div class="relative w-full overflow-hidden">
            {children()}
        </div>
    }
}

/// The scrolling viewport. Place CarouselItems directly inside.
#[component]
pub fn CarouselContent(children: Children) -> impl IntoView {
    let ctx = use_context::<CarouselCtx>().expect("CarouselContent must be inside Carousel");

    view! {
        <div class="overflow-hidden">
            <div
                class="flex transition-transform duration-300 ease-in-out"
                style=move || format!("transform: translateX(-{}%)", ctx.index.get() * 100)
            >
                {children()}
            </div>
        </div>
    }
}

/// A single slide. Full-width, shrink-0 so it never collapses.
#[component]
pub fn CarouselItem(children: Children) -> impl IntoView {
    view! {
        <div class="min-w-full shrink-0">
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselPrevious() -> impl IntoView {
    let ctx = use_context::<CarouselCtx>().expect("CarouselPrevious must be inside Carousel");

    let at_start = move || ctx.index.get() == 0;

    view! {
        <button
            class=move || {
                let base = format!("absolute start-2 top-1/2 -translate-y-1/2 z-10 flex h-8 w-8 items-center justify-center rounded-full border border-border bg-background text-foreground {} {} {}", CONTROL_MOTION, PRESSABLE, FOCUS_RING);
                if at_start() {
                    format!("{base} opacity-40 cursor-not-allowed")
                } else {
                    format!("{base} hover:bg-accent shadow-elev-sm")
                }
            }
            disabled=at_start
            on:click=move |_| {
                ctx.index.update(|i| {
                    if *i > 0 { *i -= 1; }
                });
            }
            aria-label="Previous slide"
        >
            <Icon icon=Signal::derive(|| icondata::LuChevronLeft) width="16" height="16" />
        </button>
    }
}

#[component]
pub fn CarouselNext() -> impl IntoView {
    let ctx = use_context::<CarouselCtx>().expect("CarouselNext must be inside Carousel");

    let at_end = move || ctx.index.get() + 1 >= ctx.count.get_value();

    view! {
        <button
            class=move || {
                let base = format!("absolute end-2 top-1/2 -translate-y-1/2 z-10 flex h-8 w-8 items-center justify-center rounded-full border border-border bg-background text-foreground {} {} {}", CONTROL_MOTION, PRESSABLE, FOCUS_RING);
                if at_end() {
                    format!("{base} opacity-40 cursor-not-allowed")
                } else {
                    format!("{base} hover:bg-accent shadow-elev-sm")
                }
            }
            disabled=at_end
            on:click=move |_| {
                ctx.index.update(|i| {
                    if *i + 1 < ctx.count.get_value() { *i += 1; }
                });
            }
            aria-label="Next slide"
        >
            <Icon icon=Signal::derive(|| icondata::LuChevronRight) width="16" height="16" />
        </button>
    }
}

/// Optional dot indicators.
#[component]
pub fn CarouselDots() -> impl IntoView {
    let ctx = use_context::<CarouselCtx>().expect("CarouselDots must be inside Carousel");

    view! {
        <div class="absolute bottom-2 start-0 end-0 flex justify-center gap-1.5">
            {(0..ctx.count.get_value()).map(|i| {
                view! {
                    <button
                        class=move || {
                            let base = "h-1.5 rounded-full transition-all";
                            if ctx.index.get() == i {
                                format!("{base} w-4 bg-foreground")
                            } else {
                                format!("{base} w-1.5 bg-foreground/40")
                            }
                        }
                        on:click=move |_| ctx.index.set(i)
                        aria-label=format!("Go to slide {}", i + 1)
                    />
                }
            }).collect_view()}
        </div>
    }
}
