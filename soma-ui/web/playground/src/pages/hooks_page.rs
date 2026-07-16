use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::{
    use_clipboard, use_local_storage, use_locale, use_media_query, use_theme, use_toggle,
    use_window_size,
};
use wasm_bindgen::JsCast;

#[component]
pub fn HooksPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    // Instantiate each hook
    let theme_hook = use_theme();
    let (locale_sig, set_locale) = use_locale("en", &["ar"]);
    let (stored_val, set_stored) = use_local_storage("hooks_demo_key", "");
    let is_wide = use_media_query("(min-width: 768px)");
    let (win_w, win_h) = use_window_size();
    let (toggled, toggle) = use_toggle(false);
    let clipboard = use_clipboard();

    view! {
        <div class="space-y-6 max-w-3xl">
            <div>
                <h1 class="text-2xl font-bold font-heading text-foreground">{move || s().hooks_title}</h1>
                <p class="text-muted-foreground mt-1">{move || s().hooks_subtitle}</p>
            </div>

            // use_theme
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"use_theme"</h2>
                <p class="text-xs text-muted-foreground">"Reads localStorage[\"theme\"] or prefers-color-scheme on init; applies dark/light class to <html>."</p>
                <div class="flex items-center gap-3">
                    <button
                        class="px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm hover:opacity-90 transition-opacity"
                        on:click=move |_| theme_hook.toggle.run(())
                    >
                        "Toggle Theme"
                    </button>
                    <span class="text-sm text-muted-foreground">
                        "Current: "
                        <span class="text-foreground font-mono">
                            {move || format!("{:?}", theme_hook.theme.get())}
                        </span>
                    </span>
                </div>
                <p class="text-xs font-mono text-muted-foreground">"use_theme() → UseThemeReturn { theme, toggle, set }"</p>
            </div>

            // use_locale
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"use_locale"</h2>
                <p class="text-xs text-muted-foreground">"Persists locale to localStorage; sets document.dir to rtl/ltr."</p>
                <div class="flex items-center gap-3">
                    <button
                        class="px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors"
                        on:click=move |_| set_locale.run("en".to_string())
                    >
                        "EN"
                    </button>
                    <button
                        class="px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors"
                        on:click=move |_| set_locale.run("ar".to_string())
                    >
                        "ع"
                    </button>
                    <span class="text-sm text-muted-foreground">
                        "locale: "
                        <span class="text-foreground font-mono">{move || locale_sig.get()}</span>
                        " · dir: "
                        <span class="text-foreground font-mono">
                            {move || if ["ar"].contains(&locale_sig.get().as_str()) { "rtl" } else { "ltr" }}
                        </span>
                    </span>
                </div>
                <p class="text-xs font-mono text-muted-foreground">"use_locale(default, rtl_list) → (Signal<String>, Callback<String>)"</p>
            </div>

            // use_local_storage
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"use_local_storage"</h2>
                <p class="text-xs text-muted-foreground">"Reactive string mirrored to localStorage — value survives page reload."</p>
                <input
                    type="text"
                    class="w-full px-3 py-2 rounded-md border border-border bg-background text-foreground text-sm focus:outline-none focus:ring-1 focus:ring-primary"
                    placeholder="Type something — it persists across reloads"
                    prop:value=move || stored_val.get()
                    on:input=move |e| {
                        let val = e.target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                            .map(|el| el.value())
                            .unwrap_or_default();
                        set_stored.run(val);
                    }
                />
                <p class="text-xs font-mono text-muted-foreground">"use_local_storage(key, default) → (Signal<String>, Callback<String>)"</p>
            </div>

            // use_media_query
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"use_media_query"</h2>
                <p class="text-xs text-muted-foreground">"Reactive matchMedia — resize the window to see it flip."</p>
                <div class="flex items-center gap-2">
                    <span class=move || if is_wide.get() {
                        "w-3 h-3 rounded-full bg-green-500"
                    } else {
                        "w-3 h-3 rounded-full bg-red-500"
                    } />
                    <span class="text-sm text-foreground font-mono">
                        "(min-width: 768px) → "
                        {move || if is_wide.get() { "true" } else { "false" }}
                    </span>
                </div>
                <p class="text-xs font-mono text-muted-foreground">"use_media_query(query) → Signal<bool>"</p>
            </div>

            // use_window_size
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"use_window_size"</h2>
                <p class="text-xs text-muted-foreground">"Live width × height — updates on resize."</p>
                <p class="text-lg font-mono text-foreground">
                    {move || format!("{:.0} × {:.0}", win_w.get(), win_h.get())}
                </p>
                <p class="text-xs font-mono text-muted-foreground">"use_window_size() → (Signal<f64>, Signal<f64>)"</p>
            </div>

            // use_toggle
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"use_toggle"</h2>
                <p class="text-xs text-muted-foreground">"Simple boolean toggle."</p>
                <div class="flex items-center gap-3">
                    <button
                        class="px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors"
                        on:click=move |_| toggle.run(())
                    >
                        "Toggle"
                    </button>
                    <span class="text-sm font-mono text-foreground">{move || toggled.get().to_string()}</span>
                </div>
                <p class="text-xs font-mono text-muted-foreground">"use_toggle(initial) → (Signal<bool>, Callback<()>)"</p>
            </div>

            // use_clipboard
            <div class="bg-card border border-border rounded-md p-6 space-y-3">
                <h2 class="text-sm font-semibold text-foreground">"use_clipboard"</h2>
                <p class="text-xs text-muted-foreground">"Copies text via navigator.clipboard."</p>
                <div class="flex items-center gap-3">
                    <button
                        class="px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm hover:opacity-90 transition-opacity"
                        on:click=move |_| clipboard.copy.run("Hello from soma-ui!".to_string())
                    >
                        {move || if clipboard.copied.get() { "Copied!" } else { "Copy" }}
                    </button>
                    <span class="text-xs text-muted-foreground font-mono">"\"Hello from soma-ui!\""</span>
                </div>
                <p class="text-xs font-mono text-muted-foreground">"use_clipboard() → UseClipboardReturn { copy, copied }"</p>
            </div>
        </div>
    }
}
