use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, PRESSABLE};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

fn get_dark() -> bool {
    js_sys::eval("document.documentElement.classList.contains('dark')")
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

/// Toggles the `dark` class on `document.documentElement` and persists to localStorage.
/// Uses the same js_sys::eval approach as playground/sidebar.rs for consistency.
#[component]
pub fn ThemeToggle(#[prop(default = String::new())] class: String) -> impl IntoView {
    let dark = RwSignal::new(get_dark());

    let combined = format!(
        "flex items-center justify-center w-9 h-9 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent {} {} {} {}",
        CONTROL_MOTION, PRESSABLE, FOCUS_RING, class
    );

    view! {
        <button
            class=combined
            aria-label=move || if dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
            on:click=move |_| {
                let _ = js_sys::eval(
                    "const d=document.documentElement;\
                     const isDark=d.classList.toggle('dark');\
                     localStorage.setItem('theme',isDark?'dark':'light');"
                );
                dark.set(get_dark());
            }
        >
            <Show
                when=move || dark.get()
                fallback=move || view! {
                    <span aria-hidden="true"><Icon icon=Signal::derive(|| icondata::LuMoon) width="16" height="16" /></span>
                }
            >
                <span aria-hidden="true"><Icon icon=Signal::derive(|| icondata::LuSun) width="16" height="16" /></span>
            </Show>
        </button>
    }
}
