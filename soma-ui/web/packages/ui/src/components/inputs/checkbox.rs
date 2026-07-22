use crate::components::shared::{disabled_cls, CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;
use leptos::web_sys;

#[component]
pub fn Checkbox(
    checked: RwSignal<bool>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
    let on_toggle = move |_| {
        if !disabled {
            checked.update(|v| *v = !*v);
            if let Some(cb) = on_change {
                cb.run(checked.get_untracked());
            }
        }
    };
    let on_key = move |e: web_sys::KeyboardEvent| {
        if e.key() == " " {
            e.prevent_default();
            if !disabled {
                checked.update(|v| *v = !*v);
                if let Some(cb) = on_change {
                    cb.run(checked.get_untracked());
                }
            }
        }
    };
    let disabled_class = if disabled {
        disabled_cls(true)
    } else {
        "cursor-pointer"
    };
    let combined = format!(
        "relative flex h-5 w-5 items-center justify-center rounded border hover:border-ring/60 {} {} {} {} {}",
        CONTROL_MOTION, PRESSABLE, FOCUS_RING, disabled_class, class
    );
    view! {
        <button
            type="button"
            role="checkbox"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            tabindex="0"
            class=combined
            class:bg-primary=move || checked.get()
            class:border-primary=move || checked.get()
            class:border-input=move || !checked.get()
            on:click=on_toggle
            on:keydown=on_key
        >
            <Show when=move || checked.get()>
                <Icon icon=Signal::derive(|| icondata::LuCheck) width="12" height="12" attr:class="text-primary-foreground" />
            </Show>
        </button>
    }
}
