use crate::components::shared::MENU_ITEM_CLASS;
use crate::icons::{icondata, Icon};
use leptos::portal::Portal;
use leptos::prelude::*;

// ponytail: Command palette as a centered portal dialog, reusing the dialog.rs
// Escape-close + backdrop-click pattern. Filter signal provided via context so
// CommandItem reads it without prop-drilling.
// Arrow-key roving focus is the documented upgrade path — not built here.

/// Context type to avoid ambiguity with other RwSignal<String> in the tree.
#[derive(Clone, Copy)]
struct CommandFilter(RwSignal<String>);

#[component]
pub fn Command(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let filter = RwSignal::new(String::new());
    provide_context(open);
    provide_context(CommandFilter(filter));

    let children = StoredValue::new(children);

    view! {
        <Portal>
            <Show when=move || open.get()>
                <CommandOverlay open=open filter=filter>
                    {children.with_value(|c| c())}
                </CommandOverlay>
            </Show>
        </Portal>
    }
}

#[component]
fn CommandOverlay(
    open: RwSignal<bool>,
    filter: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    // Escape closes and clears filter
    window_event_listener(leptos::ev::keydown, move |e| {
        if e.key() == "Escape" {
            open.set(false);
            filter.set(String::new());
        }
    });

    let close = move |_| {
        open.set(false);
        filter.set(String::new());
    };

    view! {
        <>
            <div class="fixed inset-0 z-40 bg-black/60 animate-fade-in" on:click=close />
            <div class="fixed left-1/2 top-[20%] -translate-x-1/2 z-50 w-full max-w-lg rounded-lg border border-border bg-card shadow-elev-lg animate-scale-in overflow-hidden">
                // Search bar
                <div class="flex items-center gap-2 border-b border-border px-3 py-2">
                    <Icon icon=Signal::derive(|| icondata::LuSearch) width="16" height="16" attr:class="text-muted-foreground shrink-0" />
                    <input
                        type="text"
                        placeholder="Type a command or search…"
                        autofocus=true
                        class="flex-1 bg-transparent text-sm text-foreground placeholder:text-muted-foreground focus:outline-none"
                        prop:value=move || filter.get()
                        on:input=move |e| filter.set(event_target_value(&e))
                    />
                </div>
                // Content
                <div class="max-h-80 overflow-y-auto p-1">
                    {children()}
                </div>
            </div>
        </>
    }
}

#[component]
pub fn CommandGroup(#[prop(into)] heading: String, children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <div class="mb-1">
            <div class="px-2 py-1.5 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                {heading}
            </div>
            {children.with_value(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandItem(
    /// Text used for case-insensitive substring filtering.
    #[prop(into)]
    keywords: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("CommandItem must be inside Command");
    let CommandFilter(filter) =
        use_context::<CommandFilter>().expect("CommandItem must be inside Command");

    let kw = StoredValue::new(keywords.to_lowercase());
    let children = StoredValue::new(children);

    let visible = move || {
        filter.get().is_empty() || kw.with_value(|k| k.contains(&filter.get().to_lowercase()))
    };

    let handle_click = move |_| {
        if let Some(cb) = on_select {
            cb.run(());
        }
        open.set(false);
        filter.set(String::new());
    };

    view! {
        <Show when=visible>
            <div
                class=MENU_ITEM_CLASS
                on:click=handle_click
            >
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}

#[component]
pub fn CommandEmpty(children: ChildrenFn) -> impl IntoView {
    let CommandFilter(filter) =
        use_context::<CommandFilter>().expect("CommandEmpty must be inside Command");

    // ponytail: CommandEmpty is always rendered; the page author decides when to show it.
    // Simplest approach: show when filter is non-empty (caller wraps in CommandGroup with items).
    // Full "no results" detection would require counting visible children — out of scope.
    let children = StoredValue::new(children);
    view! {
        <Show when=move || !filter.get().is_empty()>
            <div class="px-2 py-1.5 text-sm text-muted-foreground">
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}
