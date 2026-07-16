use leptos::prelude::*;

pub fn use_locale(
    default: &'static str,
    rtl: &'static [&'static str],
) -> (Signal<String>, Callback<String>) {
    let initial = js_sys::eval("localStorage.getItem('locale')")
        .ok()
        .and_then(|v| v.as_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| default.to_string());

    let locale = RwSignal::new(initial.clone());
    apply_locale(&initial, rtl);

    let set = Callback::new(move |code: String| {
        apply_locale(&code, rtl);
        locale.set(code);
    });

    (locale.into(), set)
}

fn apply_locale(code: &str, rtl: &[&str]) {
    let dir = if rtl.contains(&code) { "rtl" } else { "ltr" };
    let script = format!(
        "document.documentElement.dir='{}'; localStorage.setItem('locale', '{}');",
        dir, code
    );
    let _ = js_sys::eval(&script);
}
