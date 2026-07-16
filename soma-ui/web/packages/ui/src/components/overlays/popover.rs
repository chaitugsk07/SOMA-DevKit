use leptos::prelude::*;

// ponytail: click-catcher (fixed inset-0 z-40) closes the popover on outside click.
// Panel sits at z-50. No portal needed — anchored to trigger via relative/absolute.

#[component]
pub fn Popover(children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    let children = StoredValue::new(children);

    view! {
        <div class="relative inline-block">
            {children.with_value(|c| c())}
        </div>
    }
}

#[component]
pub fn PopoverTrigger(children: Children) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("PopoverTrigger must be inside Popover");

    view! {
        <div on:click=move |_| open.update(|v| *v = !*v) style="display:contents">
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverContent(children: ChildrenFn) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("PopoverContent must be inside Popover");
    let close = move |_| open.set(false);

    let children = StoredValue::new(children);

    view! {
        <Show when=move || open.get()>
            // Click-catcher behind panel
            <div class="fixed inset-0 z-40" on:click=close />
            <div class="absolute z-50 mt-2 rounded-md border border-border bg-card p-4 shadow-elev-md animate-scale-in">
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}
