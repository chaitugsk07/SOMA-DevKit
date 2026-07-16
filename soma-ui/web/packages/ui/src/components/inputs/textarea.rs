use crate::components::shared::{disabled_cls, CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

#[component]
pub fn Textarea(
    value: RwSignal<String>,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = 3)] rows: i32,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] leading_icon: Option<icondata::Icon>,
    #[prop(optional)] trailing_icon: Option<icondata::Icon>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let disabled_class = disabled_cls(disabled);
    let ps = if leading_icon.is_some() { " ps-8" } else { "" };
    let pe = if trailing_icon.is_some() { " pe-8" } else { "" };
    let combined = format!(
        "flex min-h-[80px] w-full rounded-md border border-input bg-card px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground resize shadow-elev-sm hover:border-ring/60{}{} {} {} {} {}",
        ps, pe, CONTROL_MOTION, FOCUS_RING, disabled_class, class
    );

    if leading_icon.is_none() && trailing_icon.is_none() {
        view! {
            <textarea
                class=combined
                placeholder=placeholder
                rows=rows
                disabled=disabled
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
            />
        }
        .into_any()
    } else {
        view! {
            <div class="relative w-full">
                {leading_icon.map(|ic| view! {
                    <Icon icon=Signal::derive(move || ic) attr:class="absolute start-2.5 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
                })}
                <textarea
                    class=combined
                    placeholder=placeholder
                    rows=rows
                    disabled=disabled
                    prop:value=move || value.get()
                    on:input=move |e| value.set(event_target_value(&e))
                />
                {trailing_icon.map(|ic| view! {
                    <Icon icon=Signal::derive(move || ic) attr:class="absolute end-2.5 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
                })}
            </div>
        }.into_any()
    }
}
