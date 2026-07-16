use crate::components::shared::{CONTROL_MOTION, MENU_ITEM_CLASS};
use crate::data_display::chips::{Chip, ChipVariant};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

// ponytail: options Vec<(value, label)> — avoids slot filtering complexity.
// Arrow-key roving focus is the documented upgrade path — not built here.

#[component]
pub fn MultiSelect(
    #[prop(into)] selected: RwSignal<Vec<String>>,
    options: Vec<(String, String)>,
    #[prop(optional, into)] placeholder: String,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let options = StoredValue::new(options);
    let placeholder = StoredValue::new(placeholder);

    let close = move |_| open.set(false);

    let trigger_class = format!(
        "min-h-10 w-full flex flex-wrap items-center gap-1 rounded-md border border-input bg-background px-3 py-1.5 text-sm cursor-pointer shadow-elev-sm hover:border-ring/60 focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2 focus-within:ring-offset-background {}",
        CONTROL_MOTION
    );

    view! {
        <div class="relative w-full">
            // Trigger — shows chips or placeholder
            <div
                class=trigger_class
                on:click=move |_| open.update(|v| *v = !*v)
            >
                {move || {
                    let sel = selected.get();
                    if sel.is_empty() {
                        view! {
                            <span class="text-muted-foreground">{placeholder.get_value()}</span>
                        }.into_any()
                    } else {
                        options.with_value(|opts| {
                            sel.iter().map(|v| {
                                let label = opts.iter()
                                    .find(|(val, _)| val == v)
                                    .map(|(_, l)| l.clone())
                                    .unwrap_or(v.clone());
                                let v2 = v.clone();
                                view! {
                                    <Chip
                                        variant=ChipVariant::Secondary
                                        removable=true
                                        on_remove=Callback::new(move |_| {
                                            selected.update(|s| s.retain(|x| x != &v2));
                                        })
                                    >
                                        {label}
                                    </Chip>
                                }
                            }).collect::<Vec<_>>().into_any()
                        })
                    }
                }}
                <span class="ms-auto">
                    <Icon icon=Signal::derive(|| icondata::LuChevronDown) width="16" height="16" attr:class="text-muted-foreground" />
                </span>
            </div>

            // Panel — stays open on item click
            <Show when=move || open.get()>
                <div class="fixed inset-0 z-40" on:click=close />
                <div class="absolute z-50 mt-1 w-full max-h-60 overflow-y-auto rounded-md border border-border bg-card p-1 shadow-md animate-scale-in">
                    {move || {
                        options.with_value(|opts| {
                            opts.iter().map(|(val, label)| {
                                let val = val.clone();
                                let label = label.clone();
                                let val2 = val.clone();
                                view! {
                                    <div
                                        class=MENU_ITEM_CLASS
                                        on:click=move |e| {
                                            e.stop_propagation();
                                            selected.update(|s| {
                                                if s.contains(&val) {
                                                    s.retain(|x| x != &val);
                                                } else {
                                                    s.push(val.clone());
                                                }
                                            });
                                        }
                                    >
                                        {label}
                                        <Show when=move || selected.get().contains(&val2)>
                                            <Icon icon=Signal::derive(|| icondata::LuCheck) width="14" height="14" />
                                        </Show>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        })
                    }}
                </div>
            </Show>
        </div>
    }
}
