use super::radio_button::RadioButton;
use leptos::prelude::*;

/// A group of radio buttons bound to a shared `RwSignal<String>`.
///
/// ponytail: uses an options vec rather than context/slot children to avoid
/// the complexity of a multi-component context protocol. Ceiling: not suitable
/// for options with complex children (use RadioButton directly in that case).
#[component]
pub fn RadioButtonGroup(
    /// The currently selected value.
    value: RwSignal<String>,
    /// Shared `name` attribute for the radio group (important for form semantics).
    #[prop(into)]
    name: String,
    /// `(value, label)` pairs for each option.
    options: Vec<(String, String)>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!("flex flex-col gap-2 {}", class);
    view! {
        <div class=combined>
            {options.into_iter().map(|(opt_val, opt_label)| {
                let name = name.clone();
                let sv = StoredValue::new(opt_val.clone());
                let checked = Signal::derive(move || value.get() == sv.get_value());
                // RwSignal<bool> bridge: selecting this button writes the opt_val into the group signal.
                let checked_rw = RwSignal::new(checked.get());
                let on_change = Callback::new(move |_: ()| {
                    value.set(sv.get_value());
                });
                // Keep checked_rw in sync when the group value changes from outside.
                Effect::new(move || {
                    checked_rw.set(checked.get());
                });
                view! {
                    <RadioButton
                        value=opt_val
                        name=name
                        checked=checked_rw
                        on_change=on_change
                    >
                        <span>{opt_label}</span>
                    </RadioButton>
                }
            }).collect_view()}
        </div>
    }
}
