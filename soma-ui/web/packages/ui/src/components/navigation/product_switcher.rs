use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, MENU_ITEM_CLASS};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

// ponytail: click-catcher (fixed inset-0 z-40) closes on outside click.
// window_event_listener for Escape mirrors EscapeBackdrop — auto-removed on unmount.
// Plain <a href="/{key}/"> for each entry: full-page nav, no leptos_router dep.

const PRODUCTS: &[(&str, &str)] = &[
    ("iam", "IAM"),
    ("flags", "Flags"),
    ("notify", "Notify"),
    ("audit", "Audit"),
    ("vault", "Vault"),
    ("analytics", "Analytics"),
    ("licensing", "Licensing"),
    ("observe", "Observe"),
];

/// Compact top-bar product-switcher dropdown.
///
/// Pass `current` as the product key (e.g. `"audit"`). The current product is
/// highlighted and rendered as plain text (not a link). All other products are
/// `<a href="/{key}/">` links for full-page navigation.
///
/// Rendering is unconditional — the caller decides whether to mount it
/// (dashboards gate on their `BASE` const).
#[component]
pub fn ProductSwitcher(current: &'static str) -> impl IntoView {
    let open = RwSignal::new(false);

    // Escape closes the menu — auto-removed when this component unmounts.
    window_event_listener(leptos::ev::keydown, move |e| {
        if e.key() == "Escape" {
            open.set(false);
        }
    });

    let current_label = PRODUCTS
        .iter()
        .find(|(k, _)| *k == current)
        .map(|(_, label)| *label)
        .unwrap_or(current);

    let trigger_class = format!(
        "inline-flex items-center gap-1.5 rounded-md px-2.5 py-1.5 text-sm font-medium \
         text-foreground border border-border bg-card hover:bg-accent hover:text-accent-foreground \
         {} {}",
        CONTROL_MOTION, FOCUS_RING
    );

    view! {
        <div class="relative inline-block">
            // Trigger button
            <button
                class=trigger_class
                aria-haspopup="menu"
                aria-expanded=move || open.get().to_string()
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <span>{current_label}</span>
                <span
                    aria-hidden="true"
                    class=move || format!("transition-transform duration-150{}", if open.get() { " rotate-180" } else { "" })
                >
                    <Icon
                        icon=Signal::derive(|| icondata::LuChevronDown)
                        attr:class="w-3.5 h-3.5 opacity-70"
                    />
                </span>
            </button>

            <Show when=move || open.get()>
                // Outside-click catcher
                <div class="fixed inset-0 z-40" on:click=move |_| open.set(false) />

                // Menu panel
                <div
                    role="menu"
                    class="absolute start-0 z-50 mt-1 min-w-[10rem] rounded-md border border-border bg-card p-1 shadow-elev-md animate-scale-in"
                >
                    {PRODUCTS
                        .iter()
                        .map(|(key, label)| {
                            let is_current = *key == current;
                            if is_current {
                                view! {
                                    <div
                                        role="menuitem"
                                        aria-current="page"
                                        class="flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm \
                                               font-medium text-foreground bg-accent cursor-default"
                                    >
                                        {*label}
                                    </div>
                                }.into_any()
                            } else {
                                let href = format!("/{}/", key);
                                view! {
                                    <a
                                        href=href
                                        role="menuitem"
                                        class=MENU_ITEM_CLASS
                                        on:click=move |_| open.set(false)
                                    >
                                        {*label}
                                    </a>
                                }.into_any()
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </Show>
        </div>
    }
}
