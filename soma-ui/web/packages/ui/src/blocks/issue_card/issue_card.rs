use crate::components::data_display::severity_badge::{Severity, SeverityBadge};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

/// Expandable security issue card — severity badge + title + meta line; expands to reveal children.
/// Use `IssueCard` when you want the full card chrome. Use `IssueRow` for dense list rows.
#[component]
pub fn IssueCard(
    severity: Severity,
    #[prop(into)] title: String,
    /// Short meta string: service name, resource type, time, etc.
    #[prop(optional, into)] meta: Option<String>,
    /// Whether the card is expanded by default.
    #[prop(default = false)] default_open: bool,
    #[prop(default = String::new())] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(default_open);

    let children = StoredValue::new(children);

    let card = format!(
        "bg-card border border-border rounded-xl shadow-elev-sm overflow-hidden {}",
        class
    );

    let chevron_class = move || {
        if open.get() {
            "transition-transform duration-200 rotate-180"
        } else {
            "transition-transform duration-200"
        }
    };

    view! {
        <div class=card>
            // Header row — always visible, toggles open state
            <button
                type="button"
                class="w-full flex items-start gap-3 p-4 text-start hover:bg-accent/40 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset"
                on:click=move |_| open.update(|v| *v = !*v)
                aria-expanded=move || open.get().to_string()
            >
                <SeverityBadge severity=severity class="mt-0.5 shrink-0".to_string() />
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-semibold text-foreground leading-tight">{title}</p>
                    {meta.map(|m| view! {
                        <p class="text-xs text-muted-foreground mt-0.5 truncate">{m}</p>
                    })}
                </div>
                <span class=chevron_class aria-hidden="true">
                    <Icon
                        icon=Signal::derive(|| icondata::LuChevronDown)
                        attr:class="w-4 h-4 text-muted-foreground shrink-0"
                    />
                </span>
            </button>

            // Expanded body
            <Show when=move || open.get()>
                <div class="border-t border-border px-4 py-4 text-sm text-muted-foreground animate-slide-down">
                    {children.with_value(|c| c())}
                </div>
            </Show>
        </div>
    }
}

/// Compact expandable issue row — for use inside a list or table; no card chrome.
#[component]
pub fn IssueRow(
    severity: Severity,
    #[prop(into)] title: String,
    #[prop(optional, into)] meta: Option<String>,
    #[prop(default = false)] default_open: bool,
    #[prop(default = String::new())] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(default_open);
    let children = StoredValue::new(children);

    let row = format!(
        "flex flex-col border-b border-border last:border-0 {}",
        class
    );

    let chevron_class = move || {
        if open.get() {
            "transition-transform duration-200 rotate-180"
        } else {
            "transition-transform duration-200"
        }
    };

    view! {
        <div class=row>
            <button
                type="button"
                class="flex items-center gap-3 py-2.5 text-start w-full hover:bg-accent/30 px-1 rounded transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset"
                on:click=move |_| open.update(|v| *v = !*v)
                aria-expanded=move || open.get().to_string()
            >
                <SeverityBadge severity=severity class="shrink-0".to_string() />
                <span class="flex-1 min-w-0 text-sm font-medium text-foreground truncate">{title}</span>
                {meta.map(|m| view! {
                    <span class="text-xs text-muted-foreground hidden md:inline shrink-0">{m}</span>
                })}
                <span class=chevron_class aria-hidden="true">
                    <Icon
                        icon=Signal::derive(|| icondata::LuChevronDown)
                        attr:class="w-3.5 h-3.5 text-muted-foreground shrink-0"
                    />
                </span>
            </button>
            <Show when=move || open.get()>
                <div class="pb-3 px-1 text-sm text-muted-foreground animate-slide-down">
                    {children.with_value(|c| c())}
                </div>
            </Show>
        </div>
    }
}
