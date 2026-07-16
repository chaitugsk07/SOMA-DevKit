use crate::components::shared::{CONTROL_MOTION, FOCUS_RING, MENU_ITEM_CLASS};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

// ponytail: options Vec<(value, label)> — avoids slot-based filtering complexity.
// Arrow-key roving focus is the documented upgrade path — not built here.

#[component]
pub fn Combobox(
    #[prop(into)] value: RwSignal<String>,
    options: Vec<(String, String)>,
    #[prop(optional, into)] placeholder: String,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let filter = RwSignal::new(String::new());

    let options = StoredValue::new(options);
    let placeholder = StoredValue::new(placeholder);

    let selected_label = move || {
        let v = value.get();
        if v.is_empty() {
            placeholder.get_value()
        } else {
            options.with_value(|opts| {
                opts.iter()
                    .find(|(val, _)| *val == v)
                    .map(|(_, label)| label.clone())
                    .unwrap_or(v)
            })
        }
    };

    let close = move |_| open.set(false);

    let trigger_class = format!(
        "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 text-sm text-foreground shadow-elev-sm hover:border-ring/60 {} {}",
        CONTROL_MOTION, FOCUS_RING
    );

    view! {
        <div class="relative w-full">
            // Trigger
            <button
                type="button"
                class=trigger_class
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <span class=move || {
                    if value.get().is_empty() { "text-muted-foreground" } else { "text-foreground" }
                }>
                    {selected_label}
                </span>
                <Icon icon=Signal::derive(|| icondata::LuChevronDown) width="16" height="16" />
            </button>

            // Panel
            <Show when=move || open.get()>
                <div class="fixed inset-0 z-40" on:click=close />
                <div class="absolute z-50 mt-1 w-full rounded-md border border-border bg-card shadow-md animate-scale-in">
                    // Search input
                    <div class="flex items-center border-b border-border px-2">
                        <Icon icon=Signal::derive(|| icondata::LuSearch) width="14" height="14" attr:class="text-muted-foreground shrink-0" />
                        <input
                            type="text"
                            placeholder="Search…"
                            class="flex-1 bg-transparent px-2 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none"
                            prop:value=move || filter.get()
                            on:input=move |e| filter.set(event_target_value(&e))
                        />
                    </div>
                    // Option list
                    <div class="max-h-60 overflow-y-auto p-1">
                        {move || {
                            let q = filter.get().to_lowercase();
                            options.with_value(|opts| {
                                let filtered: Vec<_> = opts.iter()
                                    .filter(|(_, label)| label.to_lowercase().contains(&q))
                                    .collect();

                                if filtered.is_empty() {
                                    view! {
                                        <div class="px-2 py-1.5 text-sm text-muted-foreground">"No results"</div>
                                    }.into_any()
                                } else {
                                    filtered.into_iter().map(|(val, label)| {
                                        let val = val.clone();
                                        let label = label.clone();
                                        let val2 = val.clone();
                                        view! {
                                            <div
                                                class=MENU_ITEM_CLASS
                                                on:click=move |_| {
                                                    value.set(val.clone());
                                                    filter.set(String::new());
                                                    open.set(false);
                                                }
                                            >
                                                {label}
                                                <Show when=move || value.get() == val2>
                                                    <Icon icon=Signal::derive(|| icondata::LuCheck) width="14" height="14" />
                                                </Show>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                }
                            })
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}
