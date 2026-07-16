use leptos::prelude::*;
mod app;
mod i18n;
mod layout;
mod pages;
mod ui;
mod viewport;

fn main() {
    let is_top = js_sys::eval("window.self === window.top")
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    if is_top {
        mount_to_body(app::ViewportCanvas);
    } else {
        // Strip the deploy subpath so the Leptos router sees "/" routes correctly.
        const BASE: &str = match option_env!("SOMA_BASE_PATH") {
            Some(v) => v,
            None => "",
        };
        if !BASE.is_empty() {
            let _ = js_sys::eval(&format!(
                "var p=location.pathname;\
                 if(p.startsWith('{BASE}')){{history.replaceState(null,'',p.slice({len})||'/'+location.search+location.hash)}}",
                len = BASE.len()
            ));
        }
        mount_to_body(app::App);
    }
}
