use crate::components::shared::MENU_ITEM_CLASS;
use crate::icons::{icondata, Icon};
use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Context type to avoid ambiguity with other RwSignal<String> in the tree.
#[derive(Clone, Copy)]
struct CommandFilter(RwSignal<String>);

#[derive(Clone, Copy)]
struct CommandEntry {
    id: usize,
    visible: Memo<bool>,
    activate: Callback<()>,
}

#[derive(Clone, Copy)]
struct CommandNavigation {
    active: RwSignal<Option<usize>>,
    entries: RwSignal<Vec<CommandEntry>>,
}

static NEXT_COMMAND_ID: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn Command(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let filter = RwSignal::new(String::new());
    let navigation = CommandNavigation {
        active: RwSignal::new(None),
        entries: RwSignal::new(Vec::new()),
    };
    provide_context(open);
    provide_context(CommandFilter(filter));
    provide_context(navigation);

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
    let navigation =
        use_context::<CommandNavigation>().expect("CommandOverlay must be inside Command");
    let input_ref = NodeRef::<leptos::html::Input>::new();

    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    let close = move |_| {
        open.set(false);
        filter.set(String::new());
        navigation.active.set(None);
    };

    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        let visible = navigation
            .entries
            .get_untracked()
            .into_iter()
            .filter(|entry| entry.visible.get_untracked())
            .collect::<Vec<_>>();

        match event.key().as_str() {
            "Escape" => {
                event.prevent_default();
                open.set(false);
                filter.set(String::new());
                navigation.active.set(None);
            }
            "ArrowDown" | "ArrowUp" => {
                if visible.is_empty() {
                    return;
                }
                event.prevent_default();
                let current = navigation
                    .active
                    .get_untracked()
                    .and_then(|id| visible.iter().position(|entry| entry.id == id));
                let next = if event.key() == "ArrowDown" {
                    current.map_or(0, |index| (index + 1) % visible.len())
                } else {
                    current.map_or(visible.len() - 1, |index| {
                        index.checked_sub(1).unwrap_or(visible.len() - 1)
                    })
                };
                navigation.active.set(Some(visible[next].id));
            }
            "Enter" => {
                let target = navigation
                    .active
                    .get_untracked()
                    .and_then(|id| visible.iter().find(|entry| entry.id == id).copied())
                    .or_else(|| visible.first().copied());
                if let Some(entry) = target {
                    event.prevent_default();
                    entry.activate.run(());
                }
            }
            _ => {}
        }
    };

    view! {
        <>
            <div class="fixed inset-0 z-40 bg-black/60 animate-fade-in" on:click=close />
            <div
                role="dialog"
                aria-modal="true"
                aria-label="Command palette"
                class="fixed left-1/2 top-[20%] z-50 w-[calc(100%-2rem)] max-w-lg -translate-x-1/2 overflow-hidden rounded-lg border border-border bg-card shadow-elev-lg animate-scale-in"
                on:keydown=handle_keydown
            >
                <div class="flex items-center gap-2 border-b border-border px-3 py-2">
                    <Icon icon=Signal::derive(|| icondata::LuSearch) width="16" height="16" attr:class="text-muted-foreground shrink-0" />
                    <input
                        node_ref=input_ref
                        type="text"
                        placeholder="Type a command or search…"
                        role="combobox"
                        aria-controls="soma-command-results"
                        aria-autocomplete="list"
                        aria-expanded="true"
                        autofocus=true
                        class="flex-1 bg-transparent text-sm text-foreground placeholder:text-muted-foreground focus:outline-none"
                        prop:value=move || filter.get()
                        on:input=move |e| {
                            filter.set(event_target_value(&e));
                            navigation.active.set(None);
                        }
                    />
                </div>
                <div id="soma-command-results" role="listbox" class="max-h-80 overflow-y-auto p-1">
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
    let navigation =
        use_context::<CommandNavigation>().expect("CommandItem must be inside Command");

    let kw = StoredValue::new(keywords.to_lowercase());
    let children = StoredValue::new(children);
    let id = NEXT_COMMAND_ID.fetch_add(1, Ordering::Relaxed);

    let visible = Memo::new(move |_| {
        filter.get().is_empty() || kw.with_value(|k| k.contains(&filter.get().to_lowercase()))
    });

    let activate = Callback::new(move |_| {
        if let Some(cb) = on_select {
            cb.run(());
        }
        open.set(false);
        filter.set(String::new());
        navigation.active.set(None);
    });

    navigation.entries.update(|entries| {
        entries.push(CommandEntry {
            id,
            visible,
            activate,
        });
    });
    on_cleanup(move || {
        navigation.entries.update(|entries| {
            entries.retain(|entry| entry.id != id);
        });
    });

    view! {
        <Show when=move || visible.get()>
            <button
                type="button"
                role="option"
                aria-selected=move || (navigation.active.get() == Some(id)).to_string()
                class=move || format!(
                    "{} w-full text-start {}",
                    MENU_ITEM_CLASS,
                    if navigation.active.get() == Some(id) {
                        "bg-accent text-accent-foreground"
                    } else {
                        ""
                    }
                )
                on:mouseenter=move |_| navigation.active.set(Some(id))
                on:click=move |_| activate.run(())
            >
                {children.with_value(|c| c())}
            </button>
        </Show>
    }
}

#[component]
pub fn CommandEmpty(children: ChildrenFn) -> impl IntoView {
    let CommandFilter(filter) =
        use_context::<CommandFilter>().expect("CommandEmpty must be inside Command");

    let navigation =
        use_context::<CommandNavigation>().expect("CommandEmpty must be inside Command");
    let children = StoredValue::new(children);
    view! {
        <Show when=move || {
            let _ = filter.get();
            !navigation.entries.get().iter().any(|entry| entry.visible.get())
        }>
            <div class="px-2 py-6 text-center text-sm text-muted-foreground">
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}
