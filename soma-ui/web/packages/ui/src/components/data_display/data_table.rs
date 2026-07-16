use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use crate::{Pagination, Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

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
) -> impl IntoView {
    let filter = RwSignal::new(String::new());
    // (col_key, ascending)
    let sort: RwSignal<Option<(String, bool)>> = RwSignal::new(None);
    let selected: RwSignal<HashSet<usize>> = RwSignal::new(HashSet::new());
    let current_page = RwSignal::new(1usize);

    let cols = StoredValue::new(columns);
    let all_rows = StoredValue::new(rows);

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

    view! {
        <div class="space-y-3">
            {move || filterable.then(|| view! {
                <input
                    class=format!("flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground {} {}", CONTROL_MOTION, FOCUS_RING)
                    type="text"
                    placeholder="Filter…"
                    prop:value=move || filter.get()
                    on:input=move |e| filter.set(event_target_value(&e))
                />
            })}

            <Table>
                <TableHeader>
                    <TableRow>
                        {move || selectable.then(|| {
                            let is_all = all_selected.get();
                            view! {
                                <TableHead class="w-10".to_string()>
                                    <input
                                        type="checkbox"
                                        class="h-4 w-4 cursor-pointer accent-primary"
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
                                                    prop:checked=move || selected.get().contains(&orig_idx)
                                                    on:change=move |_| {
                                                        selected.update(|sel| {
                                                            if sel.contains(&orig_idx) {
                                                                sel.remove(&orig_idx);
                                                            } else {
                                                                sel.insert(orig_idx);
                                                            }
                                                        });
                                                    }
                                                />
                                            </TableCell>
                                        }
                                    })}
                                    {move || {
                                        let row = row_stored.get_value();
                                        cols.get_value().into_iter().map(|col| {
                                            let val = row.get(&col.key).cloned().unwrap_or_default();
                                            view! { <TableCell>{val}</TableCell> }
                                        }).collect::<Vec<_>>()
                                    }}
                                </TableRow>
                            }
                        }
                    />
                </TableBody>
            </Table>

            {move || (page_size > 0 && total_pages.get() > 1).then(|| {
                view! {
                    <div class="flex justify-end">
                        <Pagination page=current_page total_pages=total_pages.get() />
                    </div>
                }
            })}
        </div>
    }
}
