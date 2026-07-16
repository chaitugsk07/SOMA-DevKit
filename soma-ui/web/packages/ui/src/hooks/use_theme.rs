use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

pub struct UseThemeReturn {
    pub theme: Signal<Theme>,
    pub toggle: Callback<()>,
    pub set: Callback<Theme>,
}

pub fn use_theme() -> UseThemeReturn {
    // read localStorage["theme"], else prefers-color-scheme
    let initial = {
        let stored = js_sys::eval(
            r#"
            (function() {
                var t = localStorage.getItem('theme');
                if (t === 'dark' || t === 'light') return t;
                return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
            })()
        "#,
        )
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| "dark".to_string());
        if stored == "dark" {
            Theme::Dark
        } else {
            Theme::Light
        }
    };
    let theme = RwSignal::new(initial);

    // Apply initial theme
    apply_theme(initial);

    let set = Callback::new(move |t: Theme| {
        theme.set(t);
        apply_theme(t);
    });

    let toggle = Callback::new(move |_: ()| {
        let next = if theme.get() == Theme::Dark {
            Theme::Light
        } else {
            Theme::Dark
        };
        theme.set(next);
        apply_theme(next);
    });

    UseThemeReturn {
        theme: theme.into(),
        toggle,
        set,
    }
}

fn apply_theme(t: Theme) {
    let class = if t == Theme::Dark { "dark" } else { "light" };
    let remove = if t == Theme::Dark { "light" } else { "dark" };
    let script = format!(
        "document.documentElement.classList.remove('{}'); \
         document.documentElement.classList.add('{}'); \
         localStorage.setItem('theme', '{}');",
        remove, class, class
    );
    let _ = js_sys::eval(&script);
}
