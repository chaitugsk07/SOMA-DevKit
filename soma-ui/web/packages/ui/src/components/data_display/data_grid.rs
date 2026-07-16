use super::data_table::Column;
use leptos::prelude::*;
use leptos::web_sys;
use std::collections::HashMap;

// ponytail: renders ALL rows — no virtualization. Ceiling: large datasets (>500 rows) will
// be slow. Upgrade path: leptos-virtual-list or windowed rendering.

#[component]
pub fn DataGrid(
    columns: Vec<Column>,
    rows: RwSignal<Vec<HashMap<String, String>>>,
    #[prop(default = String::from("max-h-96"))] height_class: String,
) -> impl IntoView {
    // (row_index, col_key) currently being edited
    let editing: RwSignal<Option<(usize, String)>> = RwSignal::new(None);

    let cols = StoredValue::new(columns);

    let container_class = format!(
        "overflow-auto border border-border rounded-md shadow-elev-sm {}",
        height_class
    );

    view! {
        <div class=container_class>
            <table class="w-full caption-bottom text-sm border-collapse">
                <thead class="[&_tr]:border-b">
                    <tr>
                        {move || cols.get_value().into_iter().enumerate().map(|(ci, col)| {
                            let cell_class = if ci == 0 {
                                "sticky left-0 z-20 bg-card h-10 px-2 text-start align-middle font-medium text-muted-foreground border border-border"
                            } else {
                                "h-10 px-2 text-start align-middle font-medium text-muted-foreground border border-border"
                            };
                            view! { <th class=cell_class>{col.header.clone()}</th> }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody class="[&_tr:last-child]:border-0">
                    <For
                        each=move || {
                            rows.get().into_iter().enumerate().collect::<Vec<_>>()
                        }
                        key=|(i, _)| *i
                        children=move |(row_idx, row)| {
                            let row_stored = StoredValue::new(row);
                            view! {
                                <tr class="border-b border-border transition-colors hover:bg-muted/50">
                                    {move || cols.get_value().into_iter().enumerate().map(|(ci, col)| {
                                        let col_key = col.key.clone();
                                        let editable = col.editable;

                                        let cell_class = if ci == 0 {
                                            "sticky left-0 z-10 bg-card border border-border px-2 py-1 text-sm"
                                        } else {
                                            "border border-border px-2 py-1 text-sm"
                                        };

                                        let ck_dbl = col_key.clone();
                                        let ck_is = col_key.clone();
                                        let ck_disp = col_key.clone();

                                        view! {
                                            <td
                                                class=cell_class
                                                on:dblclick=move |_| {
                                                    if editable {
                                                        editing.set(Some((row_idx, ck_dbl.clone())));
                                                    }
                                                }
                                            >
                                                <Show
                                                    when=move || editing.get() == Some((row_idx, ck_is.clone()))
                                                    fallback=move || {
                                                        let val = row_stored.get_value()
                                                            .get(&ck_disp).cloned().unwrap_or_default();
                                                        view! { <span>{val}</span> }
                                                    }
                                                >
                                                    {
                                                        let ck_init = col_key.clone();
                                                        let ck_kd = col_key.clone();
                                                        let ck_bl = col_key.clone();
                                                        let init_val = row_stored.get_value()
                                                            .get(&ck_init).cloned().unwrap_or_default();
                                                        let edit_val = RwSignal::new(init_val);
                                                        view! {
                                                            <input
                                                                class="w-full bg-transparent focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring rounded px-1"
                                                                prop:value=move || edit_val.get()
                                                                on:input=move |e| edit_val.set(event_target_value(&e))
                                                                on:keydown=move |e: web_sys::KeyboardEvent| {
                                                                    match e.key().as_str() {
                                                                        "Enter" => {
                                                                            let v = edit_val.get_untracked();
                                                                            rows.update(|rs| {
                                                                                if let Some(row) = rs.get_mut(row_idx) {
                                                                                    row.insert(ck_kd.clone(), v);
                                                                                }
                                                                            });
                                                                            editing.set(None);
                                                                        }
                                                                        "Escape" => { editing.set(None); }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                on:blur=move |_| {
                                                                    let v = edit_val.get_untracked();
                                                                    rows.update(|rs| {
                                                                        if let Some(row) = rs.get_mut(row_idx) {
                                                                            row.insert(ck_bl.clone(), v);
                                                                        }
                                                                    });
                                                                    editing.set(None);
                                                                }
                                                                autofocus=true
                                                            />
                                                        }
                                                    }
                                                </Show>
                                            </td>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}
