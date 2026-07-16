use crate::components::shared::{disabled_cls, CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional)] id: Option<String>,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = false)] disabled: bool,
    value: RwSignal<String>,
    #[prop(optional)] leading_icon: Option<icondata::Icon>,
    #[prop(optional)] trailing_icon: Option<icondata::Icon>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let disabled_class = disabled_cls(disabled);
    let ps = if leading_icon.is_some() { " ps-8" } else { "" };
    let pe = if trailing_icon.is_some() { " pe-8" } else { "" };
    let combined = format!(
        "flex h-10 w-full rounded-md border border-input bg-card px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground shadow-elev-sm hover:border-ring/60{}{} {} {} {} {}",
        ps, pe, CONTROL_MOTION, FOCUS_RING, disabled_class, class
    );

    if leading_icon.is_none() && trailing_icon.is_none() {
        view! {
            <input
                id=id
                type=input_type
                placeholder=placeholder
                disabled=disabled
                class=combined
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
            />
        }
        .into_any()
    } else {
        view! {
            <div class="relative w-full">
                {leading_icon.map(|ic| view! {
                    <Icon icon=Signal::derive(move || ic) attr:class="absolute start-2.5 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none" />
                })}
                <input
                    id=id
                    type=input_type
                    placeholder=placeholder
                    disabled=disabled
                    class=combined
                    prop:value=move || value.get()
                    on:input=move |e| value.set(event_target_value(&e))
                />
                {trailing_icon.map(|ic| view! {
                    <Icon icon=Signal::derive(move || ic) attr:class="absolute end-2.5 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none" />
                })}
            </div>
        }.into_any()
    }
}
