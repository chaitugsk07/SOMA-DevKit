use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use leptos::leptos_dom::helpers::{set_interval_with_handle, IntervalHandle};
use leptos::prelude::*;
use std::time::Duration;

/// A press-and-hold button. Hold for `duration_ms` milliseconds to fire `on_action`.
///
/// ponytail: using set_interval for progress animation (~60 fps). Interval handle
/// stored in StoredValue to avoid reactive overhead. Ceiling: ~60 fps is sufficient
/// for this use case; requestAnimationFrame would be smoother but requires more wiring.
#[component]
pub fn ButtonAction(
    on_action: Callback<()>,
    #[prop(default = 1000)] duration_ms: u32,
    #[prop(default = String::new())] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let progress = RwSignal::new(0.0f64);
    // StoredValue so we can stash/clear the handle without reactive overhead
    let handle: StoredValue<Option<IntervalHandle>> = StoredValue::new(None);

    let tick_ms = 16u32; // ~60 fps
    let increment = (tick_ms as f64 / duration_ms as f64) * 100.0;

    let start = move || {
        if handle.get_value().is_some() {
            return; // already running
        }
        let h = set_interval_with_handle(
            move || {
                let next = (progress.get() + increment).min(100.0);
                progress.set(next);
                if next >= 100.0 {
                    if let Some(h) = handle.get_value() {
                        h.clear();
                    }
                    handle.set_value(None);
                    on_action.run(());
                }
            },
            Duration::from_millis(tick_ms as u64),
        );
        handle.set_value(h.ok());
    };

    let stop = move || {
        if let Some(h) = handle.get_value() {
            h.clear();
        }
        handle.set_value(None);
        progress.set(0.0);
    };

    let combined = format!(
        "relative inline-flex items-center justify-center overflow-hidden rounded-md px-4 py-2 text-sm font-medium select-none shadow-elev-sm hover:shadow-elev {} {} {} {}",
        CONTROL_MOTION, PRESSABLE, FOCUS_RING, class
    );

    view! {
        <button
            class=combined
            on:pointerdown=move |_| start()
            on:pointerup=move |_| stop()
            on:pointerleave=move |_| stop()
        >
            // Progress fill overlay
            <span
                class="absolute inset-0 bg-primary/20 origin-left transition-none"
                style=move || format!("transform: scaleX({})", progress.get() / 100.0)
            />
            // Content sits above overlay
            <span class="relative z-10">{children()}</span>
        </button>
    }
}
