use crate::components::shared::{disabled_cls, CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use leptos::prelude::*;
use web_sys::File;

/// A styled file-picker that visually matches Button.
/// Input value is cleared after each pick so re-selecting the same file re-fires `on_file`.
#[component]
pub fn FileButton(
    #[prop(optional, into)] accept: Option<String>,
    on_file: Callback<File>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = "border border-input bg-transparent hover:bg-accent hover:text-accent-foreground";
    let size_class = "h-8 px-3 text-xs";

    let disabled_class = disabled_cls(disabled);

    let combined = format!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md font-medium {} {} {} {} {} {}",
        CONTROL_MOTION, PRESSABLE, FOCUS_RING, variant_class, size_class, disabled_class
    );
    let combined = if class.is_empty() {
        combined
    } else {
        format!("{} {}", combined, class)
    };

    let on_change = move |ev: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        let input = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    on_file.run(file);
                }
            }
            input.set_value("");
        }
    };

    view! {
        <label class=combined>
            <input
                type="file"
                class="sr-only"
                accept=accept
                disabled=disabled
                on:change=on_change
            />
            {children()}
        </label>
    }
}
