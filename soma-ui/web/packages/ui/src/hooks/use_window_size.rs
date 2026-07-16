use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn use_window_size() -> (Signal<f64>, Signal<f64>) {
    let get_w = || {
        js_sys::eval("window.innerWidth")
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    };
    let get_h = || {
        js_sys::eval("window.innerHeight")
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    };

    let width = RwSignal::new(get_w());
    let height = RwSignal::new(get_h());

    Effect::new(move |_| {
        let cb = Closure::<dyn Fn()>::new(move || {
            width.set(get_w());
            height.set(get_h());
        });

        let window = web_sys::window().expect("no window");
        let _ = window.add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref());

        let cb_fn = cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
        on_cleanup(move || {
            let w = web_sys::window().expect("no window");
            let _ = w.remove_event_listener_with_callback("resize", &cb_fn);
        });

        cb.forget();
    });

    (width.into(), height.into())
}
