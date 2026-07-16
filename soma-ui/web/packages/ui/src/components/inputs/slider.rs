use crate::components::shared::{disabled_cls, CONTROL_MOTION, FOCUS_RING};
use leptos::prelude::*;

#[component]
pub fn Slider(
    value: RwSignal<f64>,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let disabled_class = if disabled {
        disabled_cls(true)
    } else {
        "cursor-pointer"
    };
    let combined = format!(
        "w-full h-2 rounded-full bg-input accent-primary shadow-elev-sm {} {} {} {}",
        CONTROL_MOTION, FOCUS_RING, disabled_class, class
    );
    view! {
        <input
            type="range"
            min=min
            max=max
            step=step
            disabled=disabled
            class=combined
            prop:value=move || value.get().to_string()
            on:input=move |e| {
                if let Ok(v) = event_target_value(&e).parse::<f64>() {
                    value.set(v);
                }
            }
        />
    }
}
