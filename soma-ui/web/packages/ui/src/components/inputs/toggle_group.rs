use crate::components::shared::{CONTROL_MOTION, FOCUS_RING_INSET};
use leptos::prelude::*;

/// ponytail: single-select only; multi-select would need `Vec<String>` context.
#[component]
pub fn ToggleGroup(value: RwSignal<String>, children: Children) -> impl IntoView {
    provide_context(value);
    view! {
        <div class="inline-flex rounded-md border border-input overflow-hidden">
            {children()}
        </div>
    }
}

#[component]
pub fn ToggleGroupItem(#[prop(into)] value: String, children: Children) -> impl IntoView {
    let group_value =
        use_context::<RwSignal<String>>().expect("ToggleGroupItem must be used inside ToggleGroup");
    let val = StoredValue::new(value.clone());
    let item_class = format!(
        "px-4 py-2 text-sm font-medium {} {}",
        FOCUS_RING_INSET, CONTROL_MOTION
    );
    view! {
        <button
            class=item_class
            class:bg-accent=move || group_value.get() == val.get_value()
            class:shadow-elev-sm=move || group_value.get() == val.get_value()
            class:text-accent-foreground=move || group_value.get() == val.get_value()
            class:text-muted-foreground=move || group_value.get() != val.get_value()
            on:click=move |_| group_value.set(val.get_value())
        >
            {children()}
        </button>
    }
}
