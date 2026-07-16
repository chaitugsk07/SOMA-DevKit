use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn use_media_query(query: &'static str) -> Signal<bool> {
    // ponytail: using js_sys::eval for MediaQueryList to avoid web-sys MediaQueryList event listener setup complexity.
    // Ceiling: listener is tied to the Effect lifetime; on_cleanup removes it.

    let check = move || {
        js_sys::eval(&format!("window.matchMedia('{}').matches", query))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    };

    let matches = RwSignal::new(check());

    Effect::new(move |_| {
        let matches_write = matches;

        let cb = Closure::<dyn Fn(js_sys::Object)>::new(move |e: js_sys::Object| {
            let matches_val = js_sys::Reflect::get(&e, &"matches".into())
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            matches_write.set(matches_val);
        });

        let mql = js_sys::eval(&format!("window.matchMedia('{}')", query)).ok();

        if let Some(mql_val) = mql {
            if let Ok(mql_obj) = mql_val.dyn_into::<js_sys::Object>() {
                let add_fn = js_sys::Reflect::get(&mql_obj, &"addEventListener".into()).ok();
                if let Some(add_fn) = add_fn.and_then(|f| f.dyn_into::<js_sys::Function>().ok()) {
                    let _ = add_fn.call2(
                        &mql_obj,
                        &"change".into(),
                        cb.as_ref().unchecked_ref::<js_sys::Function>(),
                    );
                }

                let mql_clone = mql_obj.clone();
                let cb_fn = cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
                on_cleanup(move || {
                    let remove_fn =
                        js_sys::Reflect::get(&mql_clone, &"removeEventListener".into()).ok();
                    if let Some(remove_fn) =
                        remove_fn.and_then(|f| f.dyn_into::<js_sys::Function>().ok())
                    {
                        let _ = remove_fn.call2(&mql_clone, &"change".into(), &cb_fn);
                    }
                });
            }
        }

        cb.forget(); // ponytail: forget to avoid drop; listener cleanup handled by on_cleanup above
    });

    matches.into()
}
