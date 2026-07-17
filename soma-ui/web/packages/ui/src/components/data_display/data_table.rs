use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use crate::{
    Pagination, Table, TableBody, TableCell, TableDensity, TableHead, TableHeader, TableRow,
};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub type DataCellRenderer =
    Arc<dyn Fn(&str, &str, &HashMap<String, String>) -> AnyView + Send + Sync>;

#[derive(Clone, Debug)]
pub struct Column {
    pub key: String,
    pub header: String,
    pub sortable: bool,
    pub editable: bool,
}

#[component]
pub fn DataTable(
    columns: Vec<Column>,
    rows: Vec<HashMap<String, String>>,
    #[prop(default = false)] selectable: bool,
    #[prop(default = false)] filterable: bool,
    #[prop(default = 0usize)] page_size: usize,
    #[prop(default = TableDensity::Comfortable)] density: TableDensity,
    #[prop(default = false)] sticky_header: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional, into)] error: Option<String>,
    #[prop(default = "Filter resources…".to_string(), into)] filter_placeholder: String,
    #[prop(default = "No resources yet.".to_string(), into)] empty_message: String,
    #[prop(default = "No resources match your filters.".to_string(), into)]
    no_results_message: String,
    #[prop(optional)] toolbar: Option<ChildrenFn>,
    #[prop(optional)] cell_renderer: Option<DataCellRenderer>,
    #[prop(optional)] on_selection_change: Option<Callback<Vec<usize>>>,
) -> impl IntoView {
    let filter = RwSignal::new(String::new());
    // (col_key, ascending)
    let sort: RwSignal<Option<(String, bool)>> = RwSignal::new(None);
    let selected: RwSignal<HashSet<usize>> = RwSignal::new(HashSet::new());
    let current_page = RwSignal::new(1usize);

    let cols = StoredValue::new(columns);
    let all_rows = StoredValue::new(rows);
    let toolbar = StoredValue::new(toolbar);
    let cell_renderer = StoredValue::new(cell_renderer);
    let error = StoredValue::new(error);
    let filter_placeholder = StoredValue::new(filter_placeholder);
    let empty_message = StoredValue::new(empty_message);
    let no_results_message = StoredValue::new(no_results_message);

    // Derived: filtered + sorted rows (with original index)
    let processed = Memo::new(move |_| {
        let f = filter.get().to_lowercase();
        let s = sort.get();
        let rows = all_rows.get_value();

        let mut result: Vec<(usize, HashMap<String, String>)> = rows
            .into_iter()
            .enumerate()
            .filter(|(_, row)| {
                if f.is_empty() {
                    return true;
                }
                row.values().any(|v| v.to_lowercase().contains(&f))
            })
            .collect();

        // ponytail: sorts lexically; f64 parse is best-effort numeric fallback
        if let Some((ref key, asc)) = s {
            let k = key.clone();
            result.sort_by(|(_, a), (_, b)| {
                let av = a.get(&k).map(|s| s.as_str()).unwrap_or("");
                let bv = b.get(&k).map(|s| s.as_str()).unwrap_or("");
                let ord = match (av.parse::<f64>(), bv.parse::<f64>()) {
                    (Ok(af), Ok(bf)) => af.partial_cmp(&bf).unwrap_or(std::cmp::Ordering::Equal),
                    _ => av.cmp(bv),
                };
                if asc {
                    ord
                } else {
                    ord.reverse()
                }
            });
        }

        result
    });

    let total_pages = Memo::new(move |_| {
        if page_size == 0 {
            return 1;
        }
        let n = processed.get().len();
        if n == 0 {
            1
        } else {
            (n + page_size - 1) / page_size
        }
    });

    // clamp page to valid range so filter changes don't leave us on a phantom page
    let paged = Memo::new(move |_| {
        let rows = processed.get();
        if page_size == 0 {
            return rows;
        }
        let total = total_pages.get().max(1);
        let page = current_page.get().min(total).saturating_sub(1);
        rows.into_iter()
            .skip(page * page_size)
            .take(page_size)
            .collect::<Vec<_>>()
    });

    let all_selected = Memo::new(move |_| {
        let page_rows = paged.get();
        if page_rows.is_empty() {
            return false;
        }
        let sel = selected.get();
        page_rows.iter().all(|(i, _)| sel.contains(i))
    });

    let notify_selection = move || {
        if let Some(callback) = on_selection_change {
            let mut indices = selected.get_untracked().into_iter().collect::<Vec<_>>();
            indices.sort_unstable();
            callback.run(indices);
        }
    };

    view! {
        <div class="space-y-3">
            {(filterable || toolbar.with_value(|content| content.is_some())).then(|| view! {
                <div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
                    {filterable.then(|| view! {
                        <div class="w-full sm:max-w-sm">
                            <input
                                class=format!("flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground {} {}", CONTROL_MOTION, FOCUS_RING)
                                type="search"
                                placeholder=filter_placeholder.get_value()
                                prop:value=move || filter.get()
                                on:input=move |e| {
                                    filter.set(event_target_value(&e));
                                    current_page.set(1);
                                }
                            />
                        </div>
                    })}
                    {toolbar.with_value(|content| content.as_ref().map(|render| render()))}
                </div>
            })}

            <Table density=density sticky_header=sticky_header>
                <TableHeader>
                    <TableRow>
                        {move || selectable.then(|| {
                            let is_all = all_selected.get();
                            view! {
                                <TableHead class="w-10".to_string()>
                                    <input
                                        type="checkbox"
                                        class="h-4 w-4 cursor-pointer accent-primary"
                                        aria-label="Select all rows on this page"
                                        prop:checked=move || all_selected.get()
                                        on:change=move |_| {
                                            let page_rows = paged.get_untracked();
                                            selected.update(|sel| {
                                                if is_all {
                                                    for (i, _) in &page_rows { sel.remove(i); }
                                                } else {
                                                    for (i, _) in &page_rows { sel.insert(*i); }
                                                }
                                            });
                                            notify_selection();
                                        }
                                    />
                                </TableHead>
                            }
                        })}
                        {move || {
                            cols.get_value().into_iter().map(|col| {
                                let key = col.key.clone();
                                let header = col.header.clone();
                                let sortable = col.sortable;
                                view! {
                                    <TableHead>
                                        {if sortable {
                                            let k = key.clone();
                                            let kk = key.clone();
                                            view! {
                                                <button
                                                    class=format!("flex items-center gap-1 hover:text-foreground {} {}", CONTROL_MOTION, FOCUS_RING)
                                                    on:click=move |_| {
                                                        sort.update(|s| {
                                                            *s = Some(match s {
                                                                Some((ref ck, ref asc)) if ck == &k => (k.clone(), !*asc),
                                                                _ => (k.clone(), true),
                                                            });
                                                        });
                                                        current_page.set(1);
                                                    }
                                                >
                                                    {header.clone()}
                                                    {move || {
                                                        let k2 = kk.clone();
                                                        match sort.get() {
                                                            Some((ref ck, true)) if ck == &k2 => view! {
                                                                <Icon icon=Signal::derive(|| icondata::LuChevronUp) width="14" height="14" />
                                                            }.into_any(),
                                                            Some((ref ck, false)) if ck == &k2 => view! {
                                                                <Icon icon=Signal::derive(|| icondata::LuChevronDown) width="14" height="14" />
                                                            }.into_any(),
                                                            _ => view! {
                                                                <Icon icon=Signal::derive(|| icondata::LuChevronsUpDown) width="14" height="14" attr:class="text-muted-foreground/50" />
                                                            }.into_any(),
                                                        }
                                                    }}
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span>{header.clone()}</span> }.into_any()
                                        }}
                                    </TableHead>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {move || {
                        let column_count = cols.with_value(|value| value.len()) + usize::from(selectable);
                        if loading {
                            view! {
                                <TableRow>
                                    <td colspan=column_count class="p-8 text-center">
                                        <div class="flex items-center justify-center gap-2 text-sm text-muted-foreground" role="status">
                                            <span class="h-4 w-4 animate-spin rounded-full border-2 border-muted-foreground/30 border-t-primary" />
                                            "Loading resources…"
                                        </div>
                                    </td>
                                </TableRow>
                            }.into_any()
                        } else if let Some(message) = error.get_value() {
                            view! {
                                <TableRow>
                                    <td colspan=column_count class="p-8 text-center">
                                        <div class="text-sm text-destructive" role="alert">{message}</div>
                                    </td>
                                </TableRow>
                            }.into_any()
                        } else if all_rows.with_value(|value| value.is_empty()) {
                            let message = empty_message.get_value();
                            view! {
                                <TableRow>
                                    <td colspan=column_count class="p-8 text-center text-sm text-muted-foreground">
                                        {message}
                                    </td>
                                </TableRow>
                            }.into_any()
                        } else if paged.get().is_empty() {
                            let message = no_results_message.get_value();
                            view! {
                                <TableRow>
                                    <td colspan=column_count class="p-8 text-center text-sm text-muted-foreground">
                                        {message}
                                    </td>
                                </TableRow>
                            }.into_any()
                        } else {
                            view! {
                                <For
                                    each=move || paged.get()
                                    key=|(i, _)| *i
                                    children=move |(orig_idx, row)| {
                                        let row_stored = StoredValue::new(row);
                                        view! {
                                            <TableRow>
                                                {move || selectable.then(|| {
                                                    view! {
                                                        <TableCell class="w-10".to_string()>
                                                            <input
                                                                type="checkbox"
                                                                class="h-4 w-4 cursor-pointer accent-primary"
                                                                aria-label="Select row"
                                                                prop:checked=move || selected.get().contains(&orig_idx)
                                                                on:change=move |_| {
                                                                    selected.update(|sel| {
                                                                        if sel.contains(&orig_idx) {
                                                                            sel.remove(&orig_idx);
                                                                        } else {
                                                                            sel.insert(orig_idx);
                                                                        }
                                                                    });
                                                                    notify_selection();
                                                                }
                                                            />
                                                        </TableCell>
                                                    }
                                                })}
                                                {move || {
                                                    let row = row_stored.get_value();
                                                    cols.get_value().into_iter().map(|col| {
                                                        let value = row.get(&col.key).cloned().unwrap_or_default();
                                                        let rendered = cell_renderer.with_value(|renderer| {
                                                            renderer.as_ref().map(|render| render(&col.key, &value, &row))
                                                        });
                                                        let content = rendered.unwrap_or_else(|| view! { <>{value}</> }.into_any());
                                                        view! { <TableCell>{content}</TableCell> }
                                                    }).collect::<Vec<_>>()
                                                }}
                                            </TableRow>
                                        }
                                    }
                                />
                            }.into_any()
                        }
                    }}
                </TableBody>
            </Table>

            {move || (!loading && error.get_value().is_none() && page_size > 0 && total_pages.get() > 1).then(|| {
                view! {
                    <div class="flex justify-end">
                        <Pagination page=current_page total_pages=total_pages.get() />
                    </div>
                }
            })}
        </div>
    }
}
