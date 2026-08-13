use crate::icons::{icondata, Icon};
use leptos::prelude::*;

/// A single nav item for the Sidebar.
///
/// Accessibility: the active `<a>` element receives `aria-current="page"` so that
/// screen readers announce the current page when navigating the sidebar.
#[derive(Clone, Default)]
pub struct SidebarItem {
    pub label: String,
    pub href: String,
    pub icon: Option<icondata::Icon>,
    pub badge: Option<String>,
}

/// A collapsible group of nav items for the Sidebar.
#[derive(Clone)]
pub struct SidebarGroup {
    pub label: String,
    pub icon: Option<icondata::Icon>,
    pub items: Vec<SidebarItem>,
    /// If true, the group starts open regardless of active-path detection.
    pub default_open: bool,
}

#[component]
pub fn Sidebar(
    #[prop(into, optional)] items: Vec<SidebarItem>,
    #[prop(into, optional)] groups: Vec<SidebarGroup>,
    #[prop(into)] active_path: Signal<String>,
    #[prop(optional)] brand: Option<AnyView>,
    #[prop(default = String::new())] class: String,
    /// When true, renders only the nav items without the mobile shell (top bar,
    /// spacer, and standalone drawer).  Use this when the Sidebar is embedded
    /// inside a layout component such as ConsoleShell that manages its own
    /// mobile drawer; the parent's click handler closes the parent drawer via
    /// event bubbling so no explicit close-on-click is needed here.
    #[prop(default = false)] embed: bool,
) -> impl IntoView {
    let drawer_open = RwSignal::new(false);

    let hamburger_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"/></svg>"#;

    let desktop_class = format!(
        "hidden md:flex bg-card border-e border-border w-60 flex-col h-full shrink-0 {}",
        class
    );

    // Store items and groups so they can be used in multiple render closures.
    let items = StoredValue::new(items);
    let groups = StoredValue::new(groups);

    // Build per-group open signals. Each group starts open if default_open OR if
    // any item in the group matches the initial active_path.
    let initial_path = active_path.get_untracked();
    let group_signals: StoredValue<Vec<RwSignal<bool>>> = StoredValue::new(
        groups
            .get_value()
            .iter()
            .map(|g| {
                let active_match = g.items.iter().any(|i| i.href == initial_path);
                RwSignal::new(g.default_open || active_match)
            })
            .collect(),
    );

    // Renders the flat items (no groups).
    let render_items = move |close_on_click: bool| {
        items
            .get_value()
            .into_iter()
            .map(move |item| {
                let href_attr = item.href.clone();
                let href_cmp = item.href.clone();
                let label = item.label.clone();
                let icon = item.icon;
                let badge = item.badge.clone();
                let href_cmp2 = href_cmp.clone();
                view! {
                    <a
                        href=href_attr
                        on:click=move |_| { if close_on_click { drawer_open.set(false); } }
                        class=move || {
                            if active_path.get() == href_cmp {
                                "flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium bg-accent text-primary transition-colors border-l-[3px] border-primary [&>svg]:text-primary".to_string()
                            } else {
                                "flex items-center gap-2 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent/50 transition-colors border-l-[3px] border-transparent".to_string()
                            }
                        }
                        aria-current=move || if active_path.get() == href_cmp2 { Some("page") } else { None }
                    >
                        {icon.map(|ic| view! { <span aria-hidden="true"><Icon icon=Signal::derive(move || ic) attr:class="w-4 h-4 shrink-0" /></span> })}
                        <span class="truncate">{label}</span>
                        {badge.map(|b| view! {
                            <span class="ms-auto text-xs font-medium bg-primary/10 text-primary rounded-full px-1.5 py-0.5 leading-none">{b}</span>
                        })}
                    </a>
                }
            })
            .collect::<Vec<_>>()
    };

    // Renders grouped items.
    let render_groups = move |close_on_click: bool| {
        let sigs = group_signals.get_value();
        groups
            .get_value()
            .into_iter()
            .enumerate()
            .map(move |(idx, group)| {
                let open_sig = sigs[idx];
                let group_label = group.label.clone();
                let group_icon = group.icon;

                // A group with a single item reads as redundant (header + one item that
                // repeats it, with an odd overlapping look). Render it as a plain
                // top-level nav link instead — cleaner and more honest.
                if group.items.len() == 1 {
                    let item = group.items.into_iter().next().unwrap();
                    let href_attr = item.href.clone();
                    let href_cmp = item.href.clone();
                    let label = item.label.clone();
                    let icon = item.icon;
                    let badge = item.badge.clone();
                    let href_cmp2 = href_cmp.clone();
                    let wrap = if idx == 0 { "flex flex-col" } else { "flex flex-col mt-4" };
                    return view! {
                        <div class=wrap>
                            <a
                                href=href_attr
                                on:click=move |_| { if close_on_click { drawer_open.set(false); } }
                                class=move || {
                                    if active_path.get() == href_cmp {
                                        "flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium bg-accent text-primary transition-colors border-l-[3px] border-primary [&>svg]:text-primary".to_string()
                                    } else {
                                        "flex items-center gap-2 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent/50 transition-colors border-l-[3px] border-transparent".to_string()
                                    }
                                }
                                aria-current=move || if active_path.get() == href_cmp2 { Some("page") } else { None }
                            >
                                {icon.map(|ic| view! { <span aria-hidden="true"><Icon icon=Signal::derive(move || ic) attr:class="w-4 h-4 shrink-0" /></span> })}
                                <span class="truncate">{label}</span>
                                {badge.map(|b| view! {
                                    <span class="ms-auto text-xs font-medium bg-primary/10 text-primary rounded-full px-1.5 py-0.5 leading-none">{b}</span>
                                })}
                            </a>
                        </div>
                    }.into_any();
                }

                let group_items = StoredValue::new(group.items);

                // First group needs no top margin; the rest get spacing so groups breathe.
                // Only classes present in the prebuilt soma-ui CSS are used here.
                let group_wrap = if idx == 0 { "flex flex-col" } else { "flex flex-col mt-4" };
                view! {
                    <div class=group_wrap>
                        // Group header — a quiet, left-aligned section label that toggles collapse.
                        <button
                            class="flex items-center gap-2 px-3 py-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground hover:text-foreground transition-colors w-full justify-start text-start"
                            on:click=move |_| open_sig.update(|v| *v = !*v)
                        >
                            {group_icon.map(|ic| view! { <span aria-hidden="true"><Icon icon=Signal::derive(move || ic) attr:class="w-4 h-4 shrink-0 opacity-70" /></span> })}
                            <span class="flex-1 truncate">{group_label}</span>
                            <span class=move || {
                                if open_sig.get() {
                                    "shrink-0 transition-transform duration-200 rotate-180"
                                } else {
                                    "shrink-0 transition-transform duration-200"
                                }
                            } aria-hidden="true">
                                <Icon
                                    icon=Signal::derive(|| icondata::LuChevronDown)
                                    attr:class="w-3 h-4 opacity-50"
                                />
                            </span>
                        </button>
                        // Group items — indented under the header via a subtle left border rail.
                        <Show when=move || open_sig.get()>
                            <div class="flex flex-col gap-1 mt-1 ms-4 border-s border-border">
                                {group_items
                                    .get_value()
                                    .into_iter()
                                    .map(move |item| {
                                        let href_attr = item.href.clone();
                                        let href_cmp = item.href.clone();
                                        let label = item.label.clone();
                                        let icon = item.icon;
                                        let badge = item.badge.clone();
                                        let href_cmp2 = href_cmp.clone();
                                        view! {
                                            <a
                                                href=href_attr
                                                on:click=move |_| {
                                                    if close_on_click { drawer_open.set(false); }
                                                }
                                                class=move || {
                                                    if active_path.get() == href_cmp {
                                                        "flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium bg-accent text-primary transition-colors border-l-[3px] border-primary [&>svg]:text-primary".to_string()
                                                    } else {
                                                        "flex items-center gap-2 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent/50 transition-colors border-l-[3px] border-transparent".to_string()
                                                    }
                                                }
                                                aria-current=move || if active_path.get() == href_cmp2 { Some("page") } else { None }
                                            >
                                                {icon.map(|ic| view! { <span aria-hidden="true"><Icon icon=Signal::derive(move || ic) attr:class="w-4 h-4 shrink-0" /></span> })}
                                                <span class="truncate">{label}</span>
                                                {badge.map(|b| view! {
                                                    <span class="ms-auto text-xs font-medium bg-primary/10 text-primary rounded-full px-1.5 py-0.5 leading-none">{b}</span>
                                                })}
                                            </a>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </Show>
                    </div>
                }.into_any()
            })
            .collect::<Vec<_>>()
    };

    // Whether to render grouped or flat layout.
    let has_groups = !groups.get_value().is_empty();

    if embed {
        // Embedded mode: render only the nav items so the parent layout
        // (e.g. ConsoleShell) can display them inside its own mobile drawer.
        // The parent's <nav on:click> closes its drawer via event bubbling;
        // no explicit close-on-click needed here.
        view! {
            <div class="flex flex-col gap-0.5 p-2 flex-1 overflow-y-auto">
                {if has_groups {
                    render_groups(false).into_any()
                } else {
                    render_items(false).into_any()
                }}
            </div>
        }.into_any()
    } else {
        // Standalone mode: full layout with desktop nav + mobile shell.
        view! {
            <>
                // Desktop sidebar — hidden below md breakpoint
                <nav class=desktop_class>
                    {brand.map(|b| view! { <div class="flex items-center p-4 border-b border-border">{b}</div> })}
                    <div class="flex flex-col gap-0.5 p-2 flex-1 overflow-y-auto">
                        {if has_groups {
                            render_groups(false).into_any()
                        } else {
                            render_items(false).into_any()
                        }}
                    </div>
                </nav>

                // Mobile top bar — visible only below md
                <div class="flex md:hidden items-center justify-between px-4 h-14 bg-card border-b border-border w-full fixed top-0 start-0 z-40 shrink-0">
                    <span class="font-heading font-bold text-foreground">"Menu"</span>
                    <button
                        class="flex items-center justify-center w-8 h-8 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
                        aria-label="Open navigation menu"
                        on:click=move |_| drawer_open.set(true)
                    >
                        <span aria-hidden="true" inner_html=hamburger_svg />
                    </button>
                </div>

                // Mobile spacer so content isn't hidden under the fixed top bar
                <div class="h-14 md:hidden shrink-0"></div>

                // Mobile drawer overlay
                <Show when=move || drawer_open.get()>
                    <div class="fixed inset-0 z-50 flex">
                        <div
                            class="absolute inset-0 bg-black/50"
                            on:click=move |_| drawer_open.set(false)
                        ></div>
                        <nav class="relative z-10 bg-card w-60 h-full flex flex-col overflow-y-auto">
                            <div class="p-4 border-b border-border flex items-center justify-between">
                                <span class="font-heading font-bold text-foreground">"Menu"</span>
                                <button
                                    class="text-muted-foreground hover:text-foreground transition-colors"
                                    aria-label="Close navigation menu"
                                    on:click=move |_| drawer_open.set(false)
                                >
                                    <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor" viewBox="0 0 16 16">
                                        <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                    </svg>
                                </button>
                            </div>
                            <div class="flex flex-col gap-0.5 p-2 flex-1">
                                {if has_groups {
                                    render_groups(true).into_any()
                                } else {
                                    render_items(true).into_any()
                                }}
                            </div>
                        </nav>
                    </div>
                </Show>
            </>
        }.into_any()
    }
}
