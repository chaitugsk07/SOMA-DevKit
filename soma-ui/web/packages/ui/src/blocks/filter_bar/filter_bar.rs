use crate::components::inputs::input::Input;
use crate::icons::icondata;
use leptos::prelude::*;

/// A single select filter entry for `FilterBar`.
#[derive(Clone, Debug)]
pub struct FilterOption {
    pub value: String,
    pub label: String,
}

/// Definition for one filter in the bar.
#[derive(Clone, Debug)]
pub struct FilterDef {
    pub id: String,
    pub label: String,
    pub options: Vec<FilterOption>,
    /// The caller owns this signal — FilterBar writes to it on selection.
    pub value: RwSignal<String>,
}

/// Horizontal filter toolbar — labeled native selects + a search input.
///
/// `filters` drives the select dropdowns; `search` is the search field value.
/// All signals are caller-owned so the parent can react to changes.
#[component]
pub fn FilterBar(
    #[prop(default = vec![])] filters: Vec<FilterDef>,
    search: RwSignal<String>,
    #[prop(default = "Search...".to_string())] search_placeholder: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let select_class =
        "h-9 rounded-md border border-input bg-background px-2.5 text-sm text-foreground \
         placeholder:text-muted-foreground shadow-elev-sm transition-all duration-150 ease-out \
         focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring \
         focus-visible:ring-offset-2 focus-visible:ring-offset-background \
         hover:border-ring/60 cursor-pointer";

    let select_views: Vec<_> = filters
        .into_iter()
        .map(|filter| {
            let filter_id = filter.id.clone();
            let sig = filter.value;

            let options: Vec<_> = filter
                .options
                .into_iter()
                .map(|opt| {
                    let val = opt.value.clone();
                    let lbl = opt.label.clone();
                    view! { <option value=val>{lbl}</option> }
                })
                .collect();

            view! {
                <div class="flex items-center gap-2 shrink-0">
                    <label
                        for=filter_id.clone()
                        class="text-xs font-medium text-muted-foreground whitespace-nowrap hidden sm:inline"
                    >{filter.label}</label>
                    <select
                        id=filter_id
                        class=select_class
                        prop:value=move || sig.get()
                        on:change=move |e| sig.set(event_target_value(&e))
                    >
                        {options}
                    </select>
                </div>
            }
        })
        .collect();

    let bar = format!("flex flex-wrap items-center gap-3 {}", class);

    view! {
        <div class=bar>
            {select_views}
            // Push search to the right on wider screens
            <div class="flex-1 min-w-[160px] max-w-[280px] ms-auto">
                <Input
                    value=search
                    placeholder=search_placeholder
                    leading_icon=icondata::LuSearch
                />
            </div>
        </div>
    }
}
