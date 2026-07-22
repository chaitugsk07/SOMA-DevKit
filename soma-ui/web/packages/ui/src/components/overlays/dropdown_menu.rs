use crate::components::shared::{MENU_ITEM_CLASS, MENU_SEPARATOR_CLASS};
use leptos::portal::Portal;
use leptos::prelude::*;

// ponytail: Portal + fixed positioning lets the panel escape any overflow:auto
// scroll container (table wrapper, card, etc.) without visual clipping.
// Position is computed from the anchor div's getBoundingClientRect() when opened.
// This mirrors the Dialog pattern (leptos::portal::Portal).

#[component]
pub fn DropdownMenu(children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    // Share anchor ref with DropdownMenuContent for position computation.
    let anchor_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    provide_context(anchor_ref);

    let children = StoredValue::new(children);

    view! {
        <div class="relative inline-block" node_ref=anchor_ref>
            {children.with_value(|c| c())}
        </div>
    }
}

#[component]
pub fn DropdownMenuTrigger(children: Children) -> impl IntoView {
    let open =
        use_context::<RwSignal<bool>>().expect("DropdownMenuTrigger must be inside DropdownMenu");

    view! {
        <div on:click=move |_| open.update(|v| *v = !*v) style="display:contents">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuContent(children: ChildrenFn) -> impl IntoView {
    let open =
        use_context::<RwSignal<bool>>().expect("DropdownMenuContent must be inside DropdownMenu");
    let anchor_ref = use_context::<NodeRef<leptos::html::Div>>()
        .expect("DropdownMenuContent must be inside DropdownMenu");
    let close = move |_| open.set(false);

    let children = StoredValue::new(children);

    // Compute fixed position from anchor rect when opened (8px = mt-2 gap).
    // Fixed positioning is relative to the viewport, so it escapes any
    // overflow:auto ancestor (table wrapper, card, etc.) that clips absolute children.
    let pos_style = Signal::derive(move || {
        if !open.get() {
            return String::new();
        }
        anchor_ref
            .get_untracked()
            .map(|el| {
                let rect = el.get_bounding_client_rect();
                let vw = window().inner_width().unwrap().as_f64().unwrap_or(1920.0);
                let right_px = (vw - rect.right()).max(8.0);
                format!("top:{}px;right:{}px", rect.bottom() + 8.0, right_px)
            })
            .unwrap_or_default()
    });

    view! {
        <Show when=move || open.get()>
            <Portal>
                <div class="fixed inset-0 z-40" on:click=close />
                <div
                    class="fixed z-50 min-w-[12rem] rounded-md border border-border bg-card text-card-foreground p-1 shadow-elev-md animate-scale-in"
                    style=pos_style
                >
                    {children.with_value(|c| c())}
                </div>
            </Portal>
        </Show>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let open =
        use_context::<RwSignal<bool>>().expect("DropdownMenuItem must be inside DropdownMenu");

    let handle_click = move |_| {
        open.set(false);
        if let Some(cb) = on_click {
            cb.run(());
        }
    };

    view! {
        <div
            class=MENU_ITEM_CLASS
            on:click=handle_click
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuLabel(children: Children) -> impl IntoView {
    view! {
        <div class="px-2 py-1.5 text-xs font-semibold text-muted-foreground">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuSeparator() -> impl IntoView {
    view! {
        <div class=MENU_SEPARATOR_CLASS />
    }
}
