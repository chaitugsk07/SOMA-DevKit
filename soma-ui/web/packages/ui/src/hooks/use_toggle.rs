use leptos::prelude::*;

pub fn use_toggle(initial: bool) -> (Signal<bool>, Callback<()>) {
    let state = RwSignal::new(initial);
    let toggle = Callback::new(move |_: ()| state.update(|v| *v = !*v));
    (state.into(), toggle)
}
