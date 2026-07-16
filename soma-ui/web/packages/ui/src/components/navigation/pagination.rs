use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

/// ponytail: simple windowing — show all pages if <=7, else current±1 with ellipses.
/// Upgrade: dynamic sibling_count param for wider windows.
#[component]
pub fn Pagination(
    #[prop(into)] page: RwSignal<usize>,
    total_pages: usize,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!("flex items-center gap-1 {}", class);

    let pages = move || {
        let current = page.get();
        if total_pages <= 7 {
            (1..=total_pages).map(|p| Some(p)).collect::<Vec<_>>()
        } else {
            let mut v: Vec<Option<usize>> = Vec::new();
            let start = current.saturating_sub(1).max(2);
            let end = (current + 1).min(total_pages - 1);
            v.push(Some(1));
            if start > 2 {
                v.push(None);
            } // ellipsis
            for p in start..=end {
                v.push(Some(p));
            }
            if end < total_pages - 1 {
                v.push(None);
            } // ellipsis
            v.push(Some(total_pages));
            v
        }
    };

    let btn_base = format!(
        "inline-flex items-center justify-center h-8 w-8 rounded-md text-sm font-medium {} {} {}",
        CONTROL_MOTION, PRESSABLE, FOCUS_RING
    );
    let nav_btn_cls = format!("{} border border-input hover:bg-accent hover:text-accent-foreground disabled:opacity-50 disabled:cursor-not-allowed", btn_base);
    let nav_btn_cls2 = nav_btn_cls.clone();
    let page_active_cls = format!(
        "{} bg-primary text-primary-foreground shadow-elev-sm",
        btn_base
    );
    let page_inactive_cls = format!(
        "{} hover:bg-accent hover:text-accent-foreground border border-input",
        btn_base
    );
    let page_active_cls = StoredValue::new(page_active_cls);
    let page_inactive_cls = StoredValue::new(page_inactive_cls);

    view! {
        <nav aria-label="Pagination" class=combined>
            // Prev
            <button
                class=nav_btn_cls
                disabled=move || page.get() == 1
                on:click=move |_| page.update(|p| { if *p > 1 { *p -= 1; } })
                aria-label="Previous page"
            >
                <Icon icon=Signal::derive(|| icondata::LuChevronLeft) width="16" height="16" />
            </button>

            // Page numbers
            <For
                each=pages
                key=|item| match item { Some(p) => format!("p{}", p), None => "ellipsis".to_string() }
                children=move |item| {
                    match item {
                        None => view! {
                            <span class="h-8 w-8 flex items-center justify-center text-muted-foreground text-sm">"…"</span>
                        }.into_any(),
                        Some(p) => {
                            let p_stored = StoredValue::new(p);
                            view! {
                                <button
                                    class=move || {
                                        if page.get() == p_stored.get_value() {
                                            page_active_cls.get_value()
                                        } else {
                                            page_inactive_cls.get_value()
                                        }
                                    }
                                    on:click=move |_| page.set(p_stored.get_value())
                                    aria-current=move || if page.get() == p_stored.get_value() { "page" } else { "false" }
                                >
                                    {p}
                                </button>
                            }.into_any()
                        }
                    }
                }
            />

            // Next
            <button
                class=nav_btn_cls2
                disabled=move || page.get() == total_pages
                on:click=move |_| page.update(|p| { if *p < total_pages { *p += 1; } })
                aria-label="Next page"
            >
                <Icon icon=Signal::derive(|| icondata::LuChevronRight) width="16" height="16" />
            </button>
        </nav>
    }
}
