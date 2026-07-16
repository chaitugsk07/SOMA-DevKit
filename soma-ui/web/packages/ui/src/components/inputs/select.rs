use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, MENU_ITEM_CLASS};
use crate::icons::{icondata, Icon};
use leptos::context::Provider;
use leptos::prelude::*;

// ponytail: click-catcher (fixed inset-0 z-40) handles outside-click close.
// Arrow-key roving focus is the documented upgrade path — not built here.

#[derive(Clone, Copy)]
struct SelectCtx {
    open: RwSignal<bool>,
    value: RwSignal<String>,
    disabled: bool,
    /// Display label for the current value. Populated by the matching SelectItem.
    label: RwSignal<String>,
}

#[component]
pub fn Select(
    #[prop(optional)] id: Option<String>,
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(default = false)] disabled: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = SelectCtx {
        open: RwSignal::new(false),
        value,
        disabled,
        label: RwSignal::new(String::new()),
    };

    let placeholder = StoredValue::new(placeholder);
    let children = StoredValue::new(children);

    view! {
        <Provider value=ctx>
            <div class="relative inline-block w-full">
                <SelectTrigger id=id placeholder=placeholder.get_value() />
                {children.with_value(|c| c())}
            </div>
        </Provider>
    }
}

#[component]
fn SelectTrigger(id: Option<String>, placeholder: String) -> impl IntoView {
    let ctx = use_context::<SelectCtx>().expect("SelectTrigger must be inside Select");

    let trigger_class = format!(
        "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground shadow-elev-sm hover:border-ring/60 {} {}",
        CONTROL_MOTION, FOCUS_RING
    );
    view! {
        <button
            id=id
            type="button"
            disabled=ctx.disabled
            class=trigger_class
            on:click=move |_| ctx.open.update(|v| *v = !*v)
        >
            <span class=move || {
                if ctx.value.get().is_empty() { "text-muted-foreground" } else { "text-foreground" }
            }>
                {move || {
                    let v = ctx.value.get();
                    let l = ctx.label.get();
                    if v.is_empty() { placeholder.clone() }
                    else if !l.is_empty() { l }
                    else { v }
                }}
            </span>
            <span aria-hidden="true">
                <Icon icon=Signal::derive(|| icondata::LuChevronDown) width="16" height="16" />
            </span>
        </button>
    }
}

#[component]
pub fn SelectContent(children: ChildrenFn) -> impl IntoView {
    let ctx = use_context::<SelectCtx>().expect("SelectContent must be inside Select");
    let close = move |_| ctx.open.set(false);

    let children = StoredValue::new(children);

    view! {
        // Click-catcher backdrop — only mounted when the dropdown is open.
        <Show when=move || ctx.open.get()>
            <div class="fixed inset-0 z-40" on:click=close />
        </Show>
        // Dropdown is always in the DOM so SelectItem Effects fire for label registration
        // even before the user opens it (handles pre-selected values like role UUIDs).
        // Visually hidden via display:none when closed.
        <div
            class="absolute z-50 mt-1 w-full rounded-md border border-border bg-card p-1 shadow-md"
            style=move || if ctx.open.get() { "" } else { "display:none" }
        >
            {children.with_value(|c| c())}
        </div>
    }
}

#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    /// Human-readable label shown in the trigger after selection.
    /// If omitted, the value string is used (works when value == display text).
    #[prop(optional, into)]
    label: String,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<SelectCtx>().expect("SelectItem must be inside Select");

    let item_value = StoredValue::new(value.clone());
    let display = StoredValue::new(if label.is_empty() { value } else { label });

    // Reactively push our label into ctx whenever this item's value is selected.
    // This handles pre-selection (value set before items render) and programmatic changes.
    Effect::new(move |_| {
        if ctx.value.get() == item_value.get_value() {
            ctx.label.set(display.get_value());
        }
    });

    let handle_click = move |_| {
        ctx.value.set(item_value.get_value());
        ctx.label.set(display.get_value());
        ctx.open.set(false);
    };

    view! {
        <div
            class=MENU_ITEM_CLASS
            on:click=handle_click
        >
            {children()}
            <Show when=move || ctx.value.get() == item_value.get_value()>
                <Icon icon=Signal::derive(|| icondata::LuCheck) width="14" height="14" />
            </Show>
        </div>
    }
}
