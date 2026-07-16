use leptos::prelude::*;

pub struct UseClipboardReturn {
    pub copy: Callback<String>,
    pub copied: Signal<bool>,
}

pub fn use_clipboard() -> UseClipboardReturn {
    let copied = RwSignal::new(false);

    let copy = Callback::new(move |text: String| {
        // ponytail: no reset-after-delay timer; caller can check `copied` signal.
        // Upgrade path: use leptos::set_timeout for auto-reset.
        let script = format!(
            "navigator.clipboard.writeText('{}').then(() => {{}}).catch(() => {{}})",
            text.replace('\'', "\\'")
        );
        let _ = js_sys::eval(&script);
        copied.set(true);
    });

    UseClipboardReturn {
        copy,
        copied: copied.into(),
    }
}
