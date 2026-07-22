use crate::components::shared::{MENU_ITEM_CLASS, MENU_SEPARATOR_CLASS};
use leptos::prelude::*;
use leptos::web_sys;

// ponytail: context menu stores cursor position from contextmenu event, renders panel fixed
// at those coords. Click-catcher (fixed inset-0 z-40) handles close-on-outside-click.

#[derive(Clone, Copy)]
struct ContextMenuCtx {
    open: RwSignal<bool>,
    pos: RwSignal<(i32, i32)>,
}

#[component]
pub fn ContextMenu(children: ChildrenFn) -> impl IntoView {
    let ctx = ContextMenuCtx {
        open: RwSignal::new(false),
        pos: RwSignal::new((0, 0)),
    };
    provide_context(ctx);

    let children = StoredValue::new(children);

    view! {
        <div>
            {children.with_value(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuTrigger(children: Children) -> impl IntoView {
    let ctx =
        use_context::<ContextMenuCtx>().expect("ContextMenuTrigger must be inside ContextMenu");

    let on_context = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ctx.pos.set((ev.client_x(), ev.client_y()));
        ctx.open.set(true);
    };

    view! {
        <div on:contextmenu=on_context>
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuContent(children: ChildrenFn) -> impl IntoView {
    let ctx =
        use_context::<ContextMenuCtx>().expect("ContextMenuContent must be inside ContextMenu");
    let close = move |_| ctx.open.set(false);

    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.open.get()>
            <div class="fixed inset-0 z-40" on:click=close />
            <div
                class="fixed z-50 min-w-[12rem] rounded-md border border-border bg-card text-card-foreground p-1 shadow-elev-md animate-scale-in"
                style=move || {
                    let (x, y) = ctx.pos.get();
                    format!("left:{}px;top:{}px", x, y)
                }
            >
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}

#[component]
pub fn ContextMenuItem(
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<ContextMenuCtx>().expect("ContextMenuItem must be inside ContextMenu");

    let handle_click = move |_| {
        ctx.open.set(false);
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
pub fn ContextMenuLabel(children: Children) -> impl IntoView {
    view! {
        <div class="px-2 py-1.5 text-xs font-semibold text-muted-foreground">
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuSeparator() -> impl IntoView {
    view! {
        <div class=MENU_SEPARATOR_CLASS />
    }
}
