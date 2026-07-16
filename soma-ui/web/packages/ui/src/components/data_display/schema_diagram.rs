use crate::components::shared::FOCUS_RING;
use crate::icons::{icondata, Icon};
use leptos::prelude::*;
use std::collections::HashMap;

// ponytail: CSS-transform pan/zoom canvas with pointer-drag node repositioning.
// Edges are SVG cubic beziers recomputed as a Memo off positions.
// Touch: two-pointer pinch-zoom + single-touch pan tracked via active_pointers map.
// Ceiling: very large schemas (100+ tables) should virtualize; upgrade path = canvas 2D.

const CARD_W: f64 = 260.0;
const HEADER_H: f64 = 38.0;
const ROW_H: f64 = 30.0;
const FIT_PAD: f64 = 40.0; // padding around content on fit-to-view

const ACCENTS: &[&str] = &[
    "#6366f1", // indigo
    "#8b5cf6", // violet
    "#ec4899", // pink
    "#f59e0b", // amber
    "#10b981", // emerald
    "#3b82f6", // blue
    "#ef4444", // red
    "#14b8a6", // teal
];

/// A single column in an ERD table card.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaColumn {
    pub name: String,
    pub col_type: String,
    pub pk: bool,
    pub fk: bool,
    pub nullable: bool,
    pub unique: bool,
}

/// A table to display in the ERD canvas.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaTable {
    pub name: String,
    pub schema: Option<String>,
    pub x: f64,
    pub y: f64,
    pub columns: Vec<SchemaColumn>,
}

/// A foreign-key relation between two tables.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaRelation {
    pub from_table: String,
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
}

#[derive(Clone, Debug)]
enum DragState {
    Node {
        table_name: String,
        start_client_x: f64,
        start_client_y: f64,
        start_node_x: f64,
        start_node_y: f64,
    },
    Pan {
        start_client_x: f64,
        start_client_y: f64,
        start_pan_x: f64,
        start_pan_y: f64,
    },
}

/// Compute (zoom, pan_x, pan_y) so all content fits centered in the viewport.
/// Returns None if the content or viewport has zero size (fall back to current values).
fn compute_fit(
    vp_w: f64,
    vp_h: f64,
    tables: &[SchemaTable],
    positions: &HashMap<String, (f64, f64)>,
) -> Option<(f64, f64, f64)> {
    if vp_w <= 0.0 || vp_h <= 0.0 || tables.is_empty() {
        return None;
    }

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for t in tables {
        let (x, y) = positions.get(&t.name).copied().unwrap_or((t.x, t.y));
        let card_h = HEADER_H + t.columns.len() as f64 * ROW_H;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + CARD_W);
        max_y = max_y.max(y + card_h);
    }

    let content_w = max_x - min_x;
    let content_h = max_y - min_y;
    if content_w <= 0.0 || content_h <= 0.0 {
        return None;
    }

    let avail_w = vp_w - 2.0 * FIT_PAD;
    let avail_h = vp_h - 2.0 * FIT_PAD;
    let z = (avail_w / content_w)
        .min(avail_h / content_h)
        .clamp(0.3, 2.0);

    // Center: pan places the scaled content in the middle of the viewport.
    // After pan+scale, content top-left is at pan + min_x*z. We want that at FIT_PAD.
    let pan_x = FIT_PAD - min_x * z + (avail_w - content_w * z) * 0.5;
    let pan_y = FIT_PAD - min_y * z + (avail_h - content_h * z) * 0.5;

    Some((z, pan_x, pan_y))
}

/// Euclidean distance between two pointer positions.
#[inline]
fn pointer_dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

#[component]
pub fn SchemaDiagram(
    tables: Vec<SchemaTable>,
    relations: Vec<SchemaRelation>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let pan: RwSignal<(f64, f64)> = RwSignal::new((0.0, 0.0));
    let zoom: RwSignal<f64> = RwSignal::new(1.0);

    // Hover state: Some(table_name) when a card is being hovered.
    let hovered: RwSignal<Option<String>> = RwSignal::new(None);

    // Build initial position map from table coords
    let initial_positions: HashMap<String, (f64, f64)> = tables
        .iter()
        .map(|t| (t.name.clone(), (t.x, t.y)))
        .collect();
    let positions: RwSignal<HashMap<String, (f64, f64)>> = RwSignal::new(initial_positions);

    let drag_state: RwSignal<Option<DragState>> = RwSignal::new(None);
    // Flag set by card header pointerdown (bubbles up) so outer pointerdown skips Pan
    let node_drag_started: RwSignal<bool> = RwSignal::new(false);

    // Touch/pinch tracking: pointerId → (client_x, client_y).
    // ponytail: HashMap is fine here; at most 10 simultaneous touch points.
    let active_pointers: RwSignal<HashMap<i32, (f64, f64)>> = RwSignal::new(HashMap::new());
    // Previous pinch distance; None means we're not yet in a two-pointer gesture.
    let last_pinch_dist: RwSignal<Option<f64>> = RwSignal::new(None);

    let tables_stored = StoredValue::new(tables);
    let relations_stored = StoredValue::new(relations);

    // Precompute table-name → accent index map (stable order)
    let accent_map: HashMap<String, usize> = tables_stored
        .get_value()
        .iter()
        .enumerate()
        .map(|(i, t)| (t.name.clone(), i % ACCENTS.len()))
        .collect();
    let accent_map = StoredValue::new(accent_map);

    // NodeRef on the outer container for fit-to-view dimension reading
    let container_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Fit-to-view: reads the container size once on mount (with one retry on the next
    // microtask if the element isn't laid out yet).
    Effect::new(move |_| {
        let tables = tables_stored.get_value();
        let pos = positions.get_untracked();

        // Returns Some(()) on success, None if layout isn't ready yet.
        let try_fit = move || -> Option<()> {
            let el = container_ref.get()?;
            let w = el.client_width() as f64;
            let h = el.client_height() as f64;
            let (z, px, py) = compute_fit(w, h, &tables, &pos)?;
            zoom.set(z);
            pan.set((px, py));
            Some(())
        };

        if try_fit().is_none() {
            // Retry on next microtask if layout wasn't ready.
            let tables2 = tables_stored.get_value();
            let pos2 = positions.get_untracked();
            leptos::task::spawn_local(async move {
                // Yield to the browser so the DOM has time to lay out.
                let promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::NULL);
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

                if let Some(el) = container_ref.get() {
                    let w = el.client_width() as f64;
                    let h = el.client_height() as f64;
                    if let Some((z, px, py)) = compute_fit(w, h, &tables2, &pos2) {
                        zoom.set(z);
                        pan.set((px, py));
                    }
                }
            });
        }
    });

    // Fit-to-view closure reused by the reset button.
    let fit_view = move || {
        let tables = tables_stored.get_value();
        let pos = positions.get_untracked();
        if let Some(el) = container_ref.get() {
            let w = el.client_width() as f64;
            let h = el.client_height() as f64;
            if let Some((z, px, py)) = compute_fit(w, h, &tables, &pos) {
                zoom.set(z);
                pan.set((px, py));
            }
        }
    };

    // For edge computation: col index lookup closure stored
    let col_index_in_table = move |table_name: &str, col_name: &str| -> Option<usize> {
        tables_stored
            .get_value()
            .iter()
            .find(|t| t.name == table_name)
            .and_then(|t| t.columns.iter().position(|c| c.name == col_name))
    };
    let col_index_fn = StoredValue::new(col_index_in_table);

    // edge_paths tuple fields:
    //   (path_d, arrow_d, color, x1, y1, x2, y2, from_table, to_table, label, mid_x, mid_y)
    let edge_paths = Memo::new(move |_| {
        let pos = positions.get();
        let rels = relations_stored.get_value();
        let amap = accent_map.get_value();
        let ci_fn = col_index_fn.get_value();

        rels.into_iter()
            .filter_map(|rel| {
                let from_pos = pos.get(&rel.from_table)?;
                let to_pos = pos.get(&rel.to_table)?;
                let from_ci = ci_fn(&rel.from_table, &rel.from_column)?;
                let to_ci = ci_fn(&rel.to_table, &rel.to_column)?;

                let sx = from_pos.0;
                let sy = from_pos.1 + HEADER_H + (from_ci as f64 + 0.5) * ROW_H;
                let tx = to_pos.0;
                let ty = to_pos.1 + HEADER_H + (to_ci as f64 + 0.5) * ROW_H;

                // Nearest-side anchoring: choose exit/entry sides to avoid backward sweeps.
                let (x1, x2, src_sign, tgt_sign) = if tx + CARD_W < sx {
                    // target is fully to the left
                    (sx, tx + CARD_W, -1.0_f64, 1.0_f64)
                } else if tx > sx + CARD_W {
                    // target is fully to the right
                    (sx + CARD_W, tx, 1.0_f64, -1.0_f64)
                } else {
                    // horizontal overlap: route by center-to-center direction
                    let src_cx = sx + CARD_W * 0.5;
                    let tgt_cx = tx + CARD_W * 0.5;
                    if tgt_cx <= src_cx {
                        (sx, tx + CARD_W, -1.0_f64, 1.0_f64)
                    } else {
                        (sx + CARD_W, tx, 1.0_f64, -1.0_f64)
                    }
                };

                let y1 = sy;
                let y2 = ty;
                let dx = (x2 - x1).abs().max(60.0) * 0.5;
                let d = format!(
                    "M {x1},{y1} C {},{y1} {},{y2} {x2},{y2}",
                    x1 + src_sign * dx,
                    x2 + tgt_sign * dx,
                );

                let accent_idx = amap.get(&rel.from_table).copied().unwrap_or(0);
                let color = ACCENTS[accent_idx];

                // Arrow tip orientation: pointing inward on target entry side.
                let arrow_dx = tgt_sign * (-8.0);
                let arrow = format!(
                    "M {x2},{y2} L {},{} L {},{}",
                    x2 + arrow_dx,
                    y2 - 4.0,
                    x2 + arrow_dx,
                    y2 + 4.0,
                );

                // Label: midpoint of the two anchor points (good approximation of the
                // bezier's t=0.5 for moderate curves). Offset slightly above the path.
                let mid_x = (x1 + x2) * 0.5;
                let mid_y = (y1 + y2) * 0.5 - 6.0;
                let label = format!("{} → {}", rel.from_column, rel.to_column);

                let from_table = rel.from_table.clone();
                let to_table = rel.to_table.clone();

                Some((
                    d,
                    arrow,
                    color.to_string(),
                    x1,
                    y1,
                    x2,
                    y2,
                    from_table,
                    to_table,
                    label,
                    mid_x,
                    mid_y,
                ))
            })
            .collect::<Vec<_>>()
    });

    let host_class = format!(
        "relative overflow-hidden min-h-[600px] bg-background border border-border rounded-md {}",
        class
    );

    view! {
        <div
            node_ref=container_ref
            class=host_class
            // touch-action:none prevents the browser from hijacking two-finger pinch/pan
            // on mobile so our pointer events can handle it exclusively.
            style="touch-action: none;"
            on:wheel=move |ev: web_sys::WheelEvent| {
                ev.prevent_default();
                let delta = ev.delta_y();
                zoom.update(|z| {
                    let factor = if delta > 0.0 { 0.9 } else { 1.1 };
                    *z = (*z * factor).clamp(0.3, 2.0);
                });
            }
            on:pointermove=move |ev: web_sys::PointerEvent| {
                let pid = ev.pointer_id();
                let cx = ev.client_x() as f64;
                let cy = ev.client_y() as f64;

                // Update the active pointer position regardless of gesture type.
                active_pointers.update(|m| { m.insert(pid, (cx, cy)); });

                let ptrs = active_pointers.get_untracked();
                if ptrs.len() == 2 {
                    // Two-pointer pinch-zoom.
                    // Collect the two positions (HashMap order doesn't matter here).
                    let mut vals = ptrs.values().copied();
                    let a = vals.next().unwrap();
                    let b = vals.next().unwrap();
                    let dist = pointer_dist(a, b);

                    if let Some(prev) = last_pinch_dist.get_untracked() {
                        if prev > 0.0 {
                            let ratio = dist / prev;
                            let mid_x = (a.0 + b.0) * 0.5;
                            let mid_y = (a.1 + b.1) * 0.5;
                            zoom.update(|z| {
                                let new_z = (*z * ratio).clamp(0.3, 2.0);
                                // Keep pinch midpoint stable: adjust pan so the canvas
                                // point under the midpoint doesn't shift.
                                // canvas_pt = (mid - pan) / z  =>  new_pan = mid - canvas_pt * new_z
                                let (pan_x, pan_y) = pan.get_untracked();
                                let canvas_x = (mid_x - pan_x) / *z;
                                let canvas_y = (mid_y - pan_y) / *z;
                                pan.set((mid_x - canvas_x * new_z, mid_y - canvas_y * new_z));
                                *z = new_z;
                            });
                        }
                    }
                    last_pinch_dist.set(Some(dist));

                    // Prevent browser default (page zoom/scroll) during pinch.
                    ev.prevent_default();
                    return;
                }

                // Single pointer: fall through to drag/pan logic.
                last_pinch_dist.set(None);

                if ev.buttons() == 0 {
                    drag_state.set(None);
                    return;
                }
                match drag_state.get() {
                    Some(DragState::Node { table_name, start_client_x, start_client_y, start_node_x, start_node_y }) => {
                        let z = zoom.get();
                        let dx = (cx - start_client_x) / z;
                        let dy = (cy - start_client_y) / z;
                        let new_x = start_node_x + dx;
                        let new_y = start_node_y + dy;
                        positions.update(|map| {
                            map.insert(table_name.clone(), (new_x, new_y));
                        });
                    }
                    Some(DragState::Pan { start_client_x, start_client_y, start_pan_x, start_pan_y }) => {
                        let dx = cx - start_client_x;
                        let dy = cy - start_client_y;
                        pan.set((start_pan_x + dx, start_pan_y + dy));
                    }
                    None => {}
                }
            }
            on:pointerup=move |ev: web_sys::PointerEvent| {
                active_pointers.update(|m| { m.remove(&ev.pointer_id()); });
                last_pinch_dist.set(None);
                drag_state.set(None);
            }
            on:pointercancel=move |ev: web_sys::PointerEvent| {
                active_pointers.update(|m| { m.remove(&ev.pointer_id()); });
                last_pinch_dist.set(None);
                drag_state.set(None);
            }
            on:pointerdown=move |ev: web_sys::PointerEvent| {
                let pid = ev.pointer_id();
                let cx = ev.client_x() as f64;
                let cy = ev.client_y() as f64;
                active_pointers.update(|m| { m.insert(pid, (cx, cy)); });

                // If a node drag was just initiated by a card header, skip starting Pan.
                if node_drag_started.get() {
                    node_drag_started.set(false);
                    return;
                }

                // Only start a canvas pan for single-pointer (don't fight two-pointer pinch).
                if active_pointers.get_untracked().len() == 1 {
                    let (pan_x, pan_y) = pan.get();
                    drag_state.set(Some(DragState::Pan {
                        start_client_x: cx,
                        start_client_y: cy,
                        start_pan_x: pan_x,
                        start_pan_y: pan_y,
                    }));
                } else {
                    // Second finger down: cancel any active pan and switch to pinch mode.
                    drag_state.set(None);
                    let ptrs = active_pointers.get_untracked();
                    let mut vals = ptrs.values().copied();
                    if let (Some(a), Some(b)) = (vals.next(), vals.next()) {
                        last_pinch_dist.set(Some(pointer_dist(a, b)));
                    }
                }
            }
        >
            // Blueprint dot grid
            <div
                class="absolute inset-0 pointer-events-none"
                style="background-image: radial-gradient(circle, #94a3b8 1px, transparent 1px); background-size: 24px 24px;"
            />

            // Transform layer
            <div
                class="absolute top-0 left-0"
                style=move || {
                    let (px, py) = pan.get();
                    let z = zoom.get();
                    format!("transform: translate({px}px, {py}px) scale({z}); transform-origin: top left;")
                }
            >
                // SVG edges (behind cards)
                <svg
                    width="6000"
                    height="4000"
                    style="overflow:visible; position:absolute; top:0; left:0; z-index:0; pointer-events:none;"
                >
                    {move || {
                        let h = hovered.get();
                        let z = zoom.get();
                        // Hide edge labels when zoomed far out (they get tiny and cluttered).
                        let show_labels = z >= 0.5;
                        edge_paths.get().into_iter().map(|(d, arrow, color, x1, y1, _x2, _y2, from_table, to_table, label, mid_x, mid_y)| {
                            // Determine per-edge visibility based on hover state.
                            // ponytail: inline style string avoids extra signals per edge.
                            let (stroke_w, stroke_opacity, fill_opacity) = match &h {
                                None => (1.75_f64, 0.5_f64, 0.6_f64),
                                Some(t) if *t == from_table || *t == to_table => (2.5, 1.0, 1.0),
                                Some(_) => (1.75, 0.15, 0.15),
                            };
                            let label_opacity = match &h {
                                None => 0.7_f64,
                                Some(t) if *t == from_table || *t == to_table => 1.0,
                                Some(_) => 0.1,
                            };
                            let stroke = color.clone();
                            // Approximate label width for halo rect: 6px per char + 4px padding.
                            let label_w = label.len() as f64 * 6.0 + 8.0;
                            let label_h = 14.0_f64;
                            view! {
                                <g style=format!("transition: opacity 0.15s;")>
                                    <path
                                        d=d
                                        fill="none"
                                        stroke=stroke.clone()
                                        stroke-width=stroke_w.to_string()
                                        stroke-opacity=stroke_opacity.to_string()
                                    />
                                    // Source dot at FK column anchor
                                    <circle
                                        cx=x1 cy=y1 r="3"
                                        fill=stroke.clone()
                                        fill-opacity=fill_opacity.to_string()
                                    />
                                    // Arrowhead at target
                                    <path
                                        d=arrow
                                        fill=stroke.clone()
                                        fill-opacity=fill_opacity.to_string()
                                        stroke="none"
                                    />
                                    // FK edge label: halo rect + text, hidden when zoomed out.
                                    <Show when=move || show_labels>
                                        <g opacity=label_opacity.to_string()>
                                            // Dark halo for legibility over the dot grid.
                                            <rect
                                                x=(mid_x - label_w * 0.5).to_string()
                                                y=(mid_y - label_h + 2.0).to_string()
                                                width=label_w.to_string()
                                                height=label_h.to_string()
                                                rx="3"
                                                ry="3"
                                                fill="rgba(15,15,20,0.75)"
                                            />
                                            <text
                                                x=mid_x.to_string()
                                                y=mid_y.to_string()
                                                text-anchor="middle"
                                                dominant-baseline="auto"
                                                font-size="10"
                                                font-family="monospace"
                                                fill=stroke.clone()
                                            >
                                                {label.clone()}
                                            </text>
                                        </g>
                                    </Show>
                                </g>
                            }
                        }).collect_view()
                    }}
                </svg>

                // Table cards
                {move || {
                    let amap = accent_map.get_value();
                    tables_stored.get_value().into_iter().map(|table| {
                        let tname = table.name.clone();
                        let tname_stored = StoredValue::new(tname.clone());
                        let accent_idx = amap.get(&table.name).copied().unwrap_or(0);
                        let accent = ACCENTS[accent_idx];
                        let display_name = match &table.schema {
                            Some(s) => format!("{}.{}", s, table.name),
                            None => table.name.clone(),
                        };
                        let columns = StoredValue::new(table.columns.clone());

                        view! {
                            <div
                                style=move || {
                                    let pos = positions.get();
                                    let (x, y) = pos.get(&tname_stored.get_value()).copied().unwrap_or((0.0, 0.0));
                                    // Dim non-hovered tables when a table is hovered.
                                    let h = hovered.get();
                                    let opacity = match &h {
                                        None => 1.0_f64,
                                        Some(t) if *t == tname_stored.get_value() => 1.0,
                                        Some(_) => 0.5,
                                    };
                                    format!(
                                        "position:absolute; left:{x}px; top:{y}px; width:{CARD_W}px; z-index:1; opacity:{opacity}; transition: opacity 0.15s;"
                                    )
                                }
                                class="rounded-md overflow-hidden shadow-lg border border-border bg-card select-none"
                                on:pointerenter=move |_| {
                                    hovered.set(Some(tname_stored.get_value()));
                                }
                                on:pointerleave=move |_| {
                                    hovered.set(None);
                                }
                            >
                                // Draggable header
                                <div
                                    class="flex items-center px-3 cursor-grab"
                                    style=format!("height:{HEADER_H}px; background:{accent}; color:white;")
                                    on:pointerdown=move |ev: web_sys::PointerEvent| {
                                        let tname = tname_stored.get_value();
                                        let pos = positions.get();
                                        let (nx, ny) = pos.get(&tname).copied().unwrap_or((0.0, 0.0));
                                        node_drag_started.set(true);
                                        drag_state.set(Some(DragState::Node {
                                            table_name: tname,
                                            start_client_x: ev.client_x() as f64,
                                            start_client_y: ev.client_y() as f64,
                                            start_node_x: nx,
                                            start_node_y: ny,
                                        }));
                                    }
                                >
                                    <span class="font-semibold text-sm truncate">{display_name}</span>
                                </div>
                                // Column rows
                                {move || {
                                    columns.get_value().into_iter().map(|col| {
                                        let col_name = col.name.clone();
                                        let col_type = col.col_type.clone();
                                        let pk = col.pk;
                                        let fk = col.fk;
                                        let nullable = col.nullable;
                                        view! {
                                            <div
                                                class="flex items-center px-2 gap-1 border-t border-border/50 text-sm bg-card"
                                                style=format!("height:{ROW_H}px;")
                                            >
                                                <Show when=move || pk>
                                                    <span class="text-yellow-500 shrink-0">
                                                        <Icon icon=Signal::derive(|| icondata::LuKey) width="12" height="12" />
                                                    </span>
                                                </Show>
                                                <Show when=move || fk>
                                                    <span class="text-blue-400 shrink-0">
                                                        <Icon icon=Signal::derive(|| icondata::LuLink) width="11" height="11" />
                                                    </span>
                                                </Show>
                                                <span class="truncate text-foreground">{col_name}</span>
                                                <span class="ml-auto shrink-0 text-xs font-mono text-muted-foreground">{col_type}</span>
                                                <Show when=move || !nullable>
                                                    <span class="text-xs text-muted-foreground/70 shrink-0">"NN"</span>
                                                </Show>
                                            </div>
                                        }
                                    }).collect_view()
                                }}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            // Toolbar (top-right, above transform layer)
            <div class="absolute top-3 right-3 flex flex-col gap-1 z-10">
                <button
                    aria-label="Zoom in"
                    class=format!("flex items-center justify-center w-8 h-8 rounded-md border border-border bg-card text-foreground hover:bg-accent hover:text-accent-foreground {}", FOCUS_RING)
                    on:click=move |_| {
                        zoom.update(|z| *z = (*z * 1.2).clamp(0.3, 2.0));
                    }
                >
                    <Icon icon=Signal::derive(|| icondata::LuZoomIn) width="16" height="16" />
                </button>
                <button
                    aria-label="Zoom out"
                    class=format!("flex items-center justify-center w-8 h-8 rounded-md border border-border bg-card text-foreground hover:bg-accent hover:text-accent-foreground {}", FOCUS_RING)
                    on:click=move |_| {
                        zoom.update(|z| *z = (*z * 0.8).clamp(0.3, 2.0));
                    }
                >
                    <Icon icon=Signal::derive(|| icondata::LuZoomOut) width="16" height="16" />
                </button>
                <button
                    aria-label="Fit to view"
                    class=format!("flex items-center justify-center w-8 h-8 rounded-md border border-border bg-card text-foreground hover:bg-accent hover:text-accent-foreground {}", FOCUS_RING)
                    on:click=move |_| fit_view()
                >
                    <Icon icon=Signal::derive(|| icondata::LuRotateCcw) width="16" height="16" />
                </button>
            </div>
        </div>
    }
}
