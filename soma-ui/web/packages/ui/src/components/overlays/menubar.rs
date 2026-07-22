use crate::components::shared::{MENU_ITEM_CLASS, MENU_SEPARATOR_CLASS};
use leptos::prelude::*;

// ponytail: shared RwSignal<Option<String>> tracks which menu is open by value.
// One click-catcher rendered from Menubar when any menu is open.
// ponytail: hover-to-switch behavior — only active when menubar already has an open menu.

#[derive(Clone, Copy)]
struct MenubarCtx {
    active: RwSignal<Option<String>>,
}

#[derive(Clone)]
struct MenubarMenuCtx {
    value: StoredValue<String>,
}

#[component]
pub fn Menubar(children: ChildrenFn) -> impl IntoView {
    let ctx = MenubarCtx {
        active: RwSignal::new(None),
    };
    provide_context(ctx);

    let close = move |_| ctx.active.set(None);
    let children = StoredValue::new(children);

    view! {
        <div>
            <Show when=move || ctx.active.get().is_some()>
                <div class="fixed inset-0 z-40" on:click=close />
            </Show>
            <div class="flex items-center gap-1 rounded-md border border-border bg-card p-1">
                {children.with_value(|c| c())}
            </div>
        </div>
    }
}

#[component]
pub fn MenubarMenu(#[prop(into)] value: String, children: ChildrenFn) -> impl IntoView {
    let menu_ctx = MenubarMenuCtx {
        value: StoredValue::new(value),
    };
    provide_context(menu_ctx);

    let children = StoredValue::new(children);

    view! {
        <div class="relative">
            {children.with_value(|c| c())}
        </div>
    }
}

#[component]
pub fn MenubarTrigger(children: Children) -> impl IntoView {
    let bar_ctx = use_context::<MenubarCtx>().expect("MenubarTrigger must be inside Menubar");
    let menu_ctx =
        use_context::<MenubarMenuCtx>().expect("MenubarTrigger must be inside MenubarMenu");

    let on_click = move |_| {
        let val = menu_ctx.value.get_value();
        bar_ctx.active.update(|a| {
            if a.as_deref() == Some(&val) {
                *a = None;
            } else {
                *a = Some(val);
            }
        });
    };

    // ponytail: hover-to-switch behavior — only active when menubar already has an open menu
    let on_hover = move |_| {
        let val = menu_ctx.value.get_value();
        bar_ctx.active.update(|a| {
            if a.is_some() {
                *a = Some(val);
            }
        });
    };

    let is_open = move || bar_ctx.active.get().as_deref() == Some(&menu_ctx.value.get_value());

    view! {
        <button
            class=move || {
                let base = "px-3 py-1.5 text-sm rounded-sm text-foreground transition-colors";
                if is_open() {
                    format!("{base} bg-accent text-accent-foreground")
                } else {
                    format!("{base} hover:bg-accent hover:text-accent-foreground")
                }
            }
            on:click=on_click
            on:mouseenter=on_hover
        >
            {children()}
        </button>
    }
}

#[component]
pub fn MenubarContent(children: ChildrenFn) -> impl IntoView {
    let bar_ctx = use_context::<MenubarCtx>().expect("MenubarContent must be inside Menubar");
    let menu_ctx =
        use_context::<MenubarMenuCtx>().expect("MenubarContent must be inside MenubarMenu");

    let children = StoredValue::new(children);

    let is_open = move || bar_ctx.active.get().as_deref() == Some(&menu_ctx.value.get_value());

    view! {
        <Show when=is_open>
            <div class="absolute z-50 mt-1 min-w-[12rem] rounded-md border border-border bg-card text-card-foreground p-1 shadow-elev-md animate-scale-in">
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}

#[component]
pub fn MenubarItem(
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let bar_ctx = use_context::<MenubarCtx>().expect("MenubarItem must be inside Menubar");

    let handle_click = move |_| {
        bar_ctx.active.set(None);
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
pub fn MenubarSeparator() -> impl IntoView {
    view! {
        <div class=MENU_SEPARATOR_CLASS />
    }
}
