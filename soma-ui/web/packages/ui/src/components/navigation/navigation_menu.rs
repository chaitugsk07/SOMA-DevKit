use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

// ponytail: mirrors menubar.rs — shared RwSignal<Option<String>> in context,
// one click-catcher when any item is open, hover-to-switch when already open.

#[derive(Clone, Copy)]
struct NavMenuCtx {
    active: RwSignal<Option<String>>,
}

#[derive(Clone)]
struct NavMenuItemCtx {
    value: StoredValue<String>,
}

#[component]
pub fn NavigationMenu(children: ChildrenFn) -> impl IntoView {
    let ctx = NavMenuCtx {
        active: RwSignal::new(None),
    };
    provide_context(ctx);

    let close = move |_| ctx.active.set(None);
    let children = StoredValue::new(children);

    view! {
        <div class="relative">
            <Show when=move || ctx.active.get().is_some()>
                <div class="fixed inset-0 z-40" on:click=close />
            </Show>
            <nav class="flex items-center gap-1">
                {children.with_value(|c| c())}
            </nav>
        </div>
    }
}

#[component]
pub fn NavigationMenuItem(#[prop(into)] value: String, children: ChildrenFn) -> impl IntoView {
    let item_ctx = NavMenuItemCtx {
        value: StoredValue::new(value),
    };
    provide_context(item_ctx);

    let children = StoredValue::new(children);

    view! {
        <div class="relative">
            {children.with_value(|c| c())}
        </div>
    }
}

#[component]
pub fn NavigationMenuTrigger(children: Children) -> impl IntoView {
    let nav_ctx =
        use_context::<NavMenuCtx>().expect("NavigationMenuTrigger must be inside NavigationMenu");
    let item_ctx = use_context::<NavMenuItemCtx>()
        .expect("NavigationMenuTrigger must be inside NavigationMenuItem");

    let on_click = move |_| {
        let val = item_ctx.value.get_value();
        nav_ctx.active.update(|a| {
            if a.as_deref() == Some(&val) {
                *a = None;
            } else {
                *a = Some(val);
            }
        });
    };

    let on_hover = move |_| {
        let val = item_ctx.value.get_value();
        nav_ctx.active.update(|a| {
            if a.is_some() {
                *a = Some(val);
            }
        });
    };

    let is_open = move || nav_ctx.active.get().as_deref() == Some(&item_ctx.value.get_value());

    view! {
        <button
            class=move || {
                let base = format!("inline-flex items-center gap-1 px-4 py-2 text-sm font-medium rounded-md text-foreground {} {}", CONTROL_MOTION, FOCUS_RING);
                if is_open() {
                    format!("{base} bg-accent")
                } else {
                    format!("{base} hover:bg-accent/60")
                }
            }
            on:click=on_click
            on:mouseenter=on_hover
        >
            {children()}
            <span class=move || {
                if is_open() {
                    "transition-transform rotate-180"
                } else {
                    "transition-transform"
                }
            }>
                <Icon icon=Signal::derive(|| icondata::LuChevronDown) width="14" height="14" />
            </span>
        </button>
    }
}

#[component]
pub fn NavigationMenuContent(children: ChildrenFn) -> impl IntoView {
    let nav_ctx =
        use_context::<NavMenuCtx>().expect("NavigationMenuContent must be inside NavigationMenu");
    let item_ctx = use_context::<NavMenuItemCtx>()
        .expect("NavigationMenuContent must be inside NavigationMenuItem");

    let children = StoredValue::new(children);
    let is_open = move || nav_ctx.active.get().as_deref() == Some(&item_ctx.value.get_value());

    view! {
        <Show when=is_open>
            <div class="absolute start-0 top-full mt-1.5 z-50 min-w-[14rem] rounded-md border border-border bg-background p-2 shadow-elev-md animate-scale-in">
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}

/// A styled link inside NavigationMenuContent.
#[component]
pub fn NavigationMenuLink(
    #[prop(into)] href: String,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "block rounded-sm px-3 py-2 text-sm text-muted-foreground hover:bg-accent hover:text-foreground transition-colors {}",
        class
    );
    view! {
        <a href=href class=combined>
            {children()}
        </a>
    }
}
