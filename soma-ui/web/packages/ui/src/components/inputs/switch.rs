use crate::components::shared::{disabled_cls, CONTROL_MOTION, FOCUS_RING};
use leptos::prelude::*;
use leptos::web_sys;

#[component]
pub fn Switch(
    checked: RwSignal<bool>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let on_toggle = move |_| {
        if !disabled {
            checked.update(|v| *v = !*v);
        }
    };
    let on_key = move |e: web_sys::KeyboardEvent| {
        if e.key() == " " {
            e.prevent_default();
            if !disabled {
                checked.update(|v| *v = !*v);
            }
        }
    };
    let disabled_class = if disabled {
        disabled_cls(true)
    } else {
        "cursor-pointer"
    };
    let combined = format!(
        "relative inline-flex h-6 w-11 items-center rounded-full {} {} {} {}",
        CONTROL_MOTION, FOCUS_RING, disabled_class, class
    );
    view! {
        <button
            role="switch"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            tabindex="0"
            class=combined
            class:bg-primary=move || checked.get()
            class:bg-input=move || !checked.get()
            on:click=on_toggle
            on:keydown=on_key
        >
            <span
                class="absolute h-5 w-5 rounded-full bg-white shadow-elev-sm transition-transform"
                class:translate-x-5=move || checked.get()
                // ponytail: using inline style to handle translate-x-0.5 (dot prevents class: directive)
                style=move || if !checked.get() { "transform: translateX(0.125rem)" } else { "" }
            />
        </button>
    }
}
