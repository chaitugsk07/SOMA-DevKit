use crate::components::shared::{CONTROL_MOTION, FOCUS_RING_INSET};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

/// ponytail: single-open only; multi-open would need `Vec<String>` context instead of `Option<String>`.
#[component]
pub fn Accordion(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open_item: RwSignal<Option<String>> = RwSignal::new(None);
    provide_context(open_item);
    let combined = format!("w-full divide-y divide-border {}", class);
    view! {
        <div class=combined>{children()}</div>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(into)] title: String,
    #[prop(default = false)] open_by_default: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let open_item = use_context::<RwSignal<Option<String>>>()
        .expect("AccordionItem must be used inside Accordion");

    let val = StoredValue::new(value.clone());

    // Set default open state on mount
    if open_by_default {
        open_item.set(Some(value));
    }

    let is_open = move || open_item.get().as_deref() == Some(val.get_value().as_str());

    let on_click = move |_| {
        open_item.update(|current| {
            if current.as_deref() == Some(val.get_value().as_str()) {
                *current = None;
            } else {
                *current = Some(val.get_value());
            }
        });
    };

    view! {
        <div class="py-0">
            <button
                class=format!("flex w-full items-center justify-between py-4 text-sm font-medium text-foreground hover:text-foreground/80 {} {}", FOCUS_RING_INSET, CONTROL_MOTION)
                on:click=on_click
            >
                <span>{title}</span>
                <span class=move || if is_open() {
                    "transition-transform duration-200 rotate-180"
                } else {
                    "transition-transform duration-200"
                }>
                    <Icon icon=Signal::derive(|| icondata::LuChevronDown) width="16" height="16" />
                </span>
            </button>
            <Show when=move || is_open()>
                <div class="pb-4 text-sm text-muted-foreground animate-slide-down">
                    {children()}
                </div>
            </Show>
        </div>
    }
}
