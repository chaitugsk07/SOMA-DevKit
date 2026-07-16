use crate::components::shared::CONTROL_MOTION;
use crate::icons::{icondata, Icon};
use crate::{Button, ButtonSize, ButtonVariant};
use leptos::prelude::*;
use leptos::web_sys;

// ponytail: auto-grow via CSS `field-sizing: content` (modern, zero JS).
// Ceiling: field-sizing is not yet in all browsers; JS scrollHeight resize is the fallback.
// Enter sends, Shift+Enter inserts newline — handled in on:keydown.
// ponytail: on_submit is Option<Callback<String>> so the component works with no handler.

#[component]
pub fn InputPrompt(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = String::from("Message..."))] placeholder: String,
    #[prop(optional)] on_submit: Option<Callback<String>>,
    #[prop(default = false)] show_attach: bool,
) -> impl IntoView {
    let placeholder = StoredValue::new(placeholder);
    let on_submit = StoredValue::new(on_submit);

    let submit = move || {
        let v = value.get();
        let trimmed = v.trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        if let Some(cb) = on_submit.get_value() {
            cb.run(trimmed);
        }
        value.set(String::new());
    };

    let on_keydown = move |e: web_sys::KeyboardEvent| {
        if e.key() == "Enter" && !e.shift_key() {
            e.prevent_default();
            submit();
        }
    };

    let outer_class = format!(
        "w-full rounded-xl border border-input bg-background px-3 pb-2 pt-3 shadow-elev-sm hover:border-ring/60 focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2 focus-within:ring-offset-background {}",
        CONTROL_MOTION
    );

    view! {
        <div class=outer_class>
            <textarea
                class="w-full resize-none bg-transparent text-sm text-foreground placeholder:text-muted-foreground focus:outline-none [field-sizing:content] min-h-[2.5rem] max-h-48 overflow-y-auto"
                placeholder=move || placeholder.get_value()
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
                on:keydown=on_keydown
            />
            <div class="flex items-center justify-between mt-1">
                <div class="flex gap-1">
                    {move || {
                        if show_attach {
                            view! {
                                <button
                                    type="button"
                                    class="flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
                                    aria-label="Attach file"
                                >
                                    <Icon icon=Signal::derive(|| icondata::LuPaperclip) width="16" height="16" />
                                </button>
                            }.into_any()
                        } else {
                            view! { <span /> }.into_any()
                        }
                    }}
                </div>
                <Button
                    variant=ButtonVariant::Default
                    size=ButtonSize::Icon
                    aria_label="Send".to_string()
                    on:click=move |_| submit()
                >
                    <Icon icon=Signal::derive(|| icondata::LuArrowUp) width="16" height="16" />
                </Button>
            </div>
        </div>
    }
}
