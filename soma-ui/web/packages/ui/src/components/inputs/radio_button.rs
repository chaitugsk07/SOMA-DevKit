use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use leptos::prelude::*;

/// A single radio button with a styled circle indicator and a label slot.
///
/// For a group of radio buttons sharing a value, use `RadioButtonGroup`.
#[component]
pub fn RadioButton(
    /// The value this radio button represents (used as the input's `value` attribute).
    #[prop(into)]
    value: String,
    /// The `name` attribute shared across a radio group.
    #[prop(into)]
    name: String,
    /// Whether this radio button is currently selected.
    checked: RwSignal<bool>,
    /// Called when this radio button is selected.
    #[prop(optional)]
    on_change: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let label_class = format!(
        "flex items-center gap-2 cursor-pointer select-none text-sm text-foreground {} {}",
        CONTROL_MOTION, PRESSABLE
    );
    let sr_class = format!("sr-only {}", FOCUS_RING);

    view! {
        <label class=label_class>
            // Hidden native radio for semantics / keyboard / form submission
            <input
                type="radio"
                class=sr_class
                value=value
                name=name
                prop:checked=move || checked.get()
                on:change=move |_| {
                    checked.set(true);
                    if let Some(cb) = on_change {
                        cb.run(());
                    }
                }
            />
            // Visible styled circle
            <span
                class="flex h-4 w-4 items-center justify-center rounded-full border transition-colors hover:border-ring/60"
                class:border-primary=move || checked.get()
                class:border-input=move || !checked.get()
            >
                <Show when=move || checked.get()>
                    <span class="h-2 w-2 rounded-full bg-primary" />
                </Show>
            </span>
            {children()}
        </label>
    }
}
