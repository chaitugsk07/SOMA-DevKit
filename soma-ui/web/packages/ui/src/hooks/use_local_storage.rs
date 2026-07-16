use leptos::prelude::*;

pub fn use_local_storage(
    key: &'static str,
    default: &'static str,
) -> (Signal<String>, Callback<String>) {
    let initial = js_sys::eval(&format!("localStorage.getItem('{}') ?? '{}'", key, default))
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| default.to_string());

    let val = RwSignal::new(initial);

    let set = Callback::new(move |s: String| {
        let _ = js_sys::eval(&format!(
            "localStorage.setItem('{}', '{}')",
            key,
            s.replace('\'', "\\'")
        ));
        val.set(s);
    });

    (val.into(), set)
}
