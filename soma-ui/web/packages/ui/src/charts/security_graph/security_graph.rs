use crate::components::data_display::severity_badge::Severity;
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

const NODE_R: f64 = 28.0;
const LABEL_OFFSET: f64 = 38.0;
const SUBLABEL_OFFSET: f64 = 50.0;
const VIEW_PAD: f64 = 90.0;

/// A node in the security graph.
#[derive(Clone, Debug)]
pub struct GraphNode {
    pub id: String,
    /// Abstract layout X coordinate (caller controls positioning).
    pub x: f64,
    pub y: f64,
    /// Bold label rendered below the circle.
    pub label: String,
    /// Smaller secondary text below the label.
    pub sublabel: String,
    /// Used to choose the ring color. Recognized values: vm, storage, network,
    /// iam, database, container, secret. Anything else → indigo.
    pub category: String,
    /// When set, the severity hue overrides the ring color and adds a badge.
    pub severity: Option<Severity>,
}

/// A directed edge between two nodes.
#[derive(Clone, Debug)]
pub struct GraphEdge {
    pub src_id: String,
    pub dst_id: String,
    /// Short label shown near the midpoint of the edge.
    pub label: String,
}

fn category_color(cat: &str) -> &'static str {
    match cat.to_lowercase().as_str() {
        "vm" | "virtual_machine" | "compute" => "#3b82f6",
        "storage" => "#f59e0b",
        "network" => "#8b5cf6",
        "iam" | "identity" => "#ec4899",
        "database" | "db" | "sql" => "#10b981",
        "container" | "kubernetes" | "k8s" => "#06b6d4",
        "secret" | "secrets" | "keyvault" => "#f43f5e",
        _ => "#6366f1",
    }
}

fn category_icon(cat: &str) -> icondata::Icon {
    match cat.to_lowercase().as_str() {
        "vm" | "virtual_machine" | "compute" | "container" | "kubernetes" | "k8s" => {
            icondata::LuServer
        }
        "storage" | "database" | "db" | "sql" | "data" => icondata::LuDatabase,
        "network" => icondata::LuGlobe,
        "iam" | "identity" => icondata::LuUser,
        "secret" | "secrets" | "keyvault" => icondata::LuKey,
        "code" => icondata::LuGitBranch,
        "ai" => icondata::LuSparkles,
        "finding" => icondata::LuShieldAlert,
        _ => icondata::LuBox,
    }
}

fn view_box_str(nodes: &[GraphNode]) -> String {
    if nodes.is_empty() {
        return "0 0 400 300".to_string();
    }
    let min_x = nodes.iter().map(|n| n.x).fold(f64::INFINITY, f64::min);
    let min_y = nodes.iter().map(|n| n.y).fold(f64::INFINITY, f64::min);
    let max_x = nodes.iter().map(|n| n.x).fold(f64::NEG_INFINITY, f64::max);
    let max_y = nodes.iter().map(|n| n.y).fold(f64::NEG_INFINITY, f64::max);
    let w = (max_x - min_x + 2.0 * VIEW_PAD).max(200.0);
    // Extra vertical pad for labels below nodes
    let h = (max_y - min_y + 2.0 * VIEW_PAD + 60.0).max(150.0);
    format!(
        "{:.1} {:.1} {:.1} {:.1}",
        min_x - VIEW_PAD,
        min_y - VIEW_PAD,
        w,
        h,
    )
}

/// Interactive SVG security-topology graph.
///
/// Renders big circular nodes with category-colored rings, directed labeled edges,
/// and supports pan (mouse drag) and zoom (scroll wheel). Notifies via `on_node_click`
/// when a node circle is clicked.
///
/// # Layout
/// Caller controls `(x, y)` positioning of nodes — no auto-layout.
/// For a clean horizontal attack chain, place nodes at `y = 0` and x increasing by ~120 per node.
#[component]
pub fn SecurityGraph(
    #[prop(default = vec![])] nodes: Vec<GraphNode>,
    #[prop(default = vec![])] edges: Vec<GraphEdge>,
    /// Called with the clicked node's `id`. `Callback<String>` is `Copy` — capture freely.
    #[prop(optional)]
    on_node_click: Option<Callback<String>>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    // ── Reactive pan / zoom ───────────────────────────────────────────────────
    let pan_x = RwSignal::new(0.0_f64);
    let pan_y = RwSignal::new(0.0_f64);
    let zoom = RwSignal::new(1.0_f64);
    let dragging = RwSignal::new(false);
    let last_xy = RwSignal::new((0.0_f64, 0.0_f64));

    let vb = view_box_str(&nodes);

    // Store node list for edge endpoint lookups (clone once into StoredValue).
    let nodes_sv = StoredValue::new(nodes);

    // ── Edges (rendered first so nodes appear on top) ─────────────────────────
    let edge_views: Vec<_> = edges
        .into_iter()
        .map(|edge| {
            let all = nodes_sv.get_value();
            let src = all.iter().find(|n| n.id == edge.src_id).cloned();
            let dst = all.iter().find(|n| n.id == edge.dst_id).cloned();

            let (src, dst) = match (src, dst) {
                (Some(s), Some(d)) => (s, d),
                _ => return view! { <g /> }.into_any(),
            };

            let dx = dst.x - src.x;
            let dy = dst.y - src.y;
            let mag = (dx * dx + dy * dy).sqrt().max(0.001);
            let ux = dx / mag;
            let uy = dy / mag;

            let x1 = src.x + ux * (NODE_R + 3.0);
            let y1 = src.y + uy * (NODE_R + 3.0);
            let x2 = dst.x - ux * (NODE_R + 10.0);
            let y2 = dst.y - uy * (NODE_R + 10.0);
            let mid_x = (x1 + x2) / 2.0;
            let mid_y = (y1 + y2) / 2.0;
            let lbl = edge.label.clone();

            view! {
                <g>
                    <line
                        x1=format!("{x1:.1}") y1=format!("{y1:.1}")
                        x2=format!("{x2:.1}") y2=format!("{y2:.1}")
                        stroke="#9ca3af"
                        stroke-width="1.5"
                        marker-end="url(#sg-arrow)"
                    />
                    <text
                        x=format!("{mid_x:.1}")
                        y=format!("{:.1}", mid_y - 6.0)
                        text-anchor="middle"
                        font-size="8"
                        fill="#9ca3af"
                    >{lbl}</text>
                </g>
            }
            .into_any()
        })
        .collect();

    // ── Nodes ────────────────────────────────────────────────────────────────
    // `on_node_click: Option<Callback<String>>` — `Callback<String>` is Copy,
    // so `Option<Callback<String>>` is Copy and can be captured by each node closure.
    let node_views: Vec<_> = nodes_sv
        .get_value()
        .into_iter()
        .map(|node| {
            let ring_color = node
                .severity
                .map(|s| s.hex())
                .unwrap_or_else(|| category_color(&node.category));

            let fill_color = format!("{}1a", ring_color); // ~10% opacity (1a = 26 decimal)
            let glow_color = format!("{}0d", ring_color); // ~5% outer glow

            let cat_icon = category_icon(&node.category);
            let label_text = node.label.clone();
            let sublabel_text = node.sublabel.clone();
            let node_id = node.id.clone();
            let nx = node.x;
            let ny = node.y;

            let sev_badge = node.severity.map(|sev| {
                let badge_color = sev.hex();
                let badge_label = sev.label();
                view! {
                    <g transform=format!("translate(0,{:.1})", SUBLABEL_OFFSET + 14.0)>
                        <rect x="-18" y="-6" width="36" height="12" rx="3" fill=badge_color />
                        <text
                            x="0" y="0"
                            text-anchor="middle"
                            dominant-baseline="central"
                            font-size="7"
                            font-weight="700"
                            fill="white"
                        >{badge_label}</text>
                    </g>
                }
            });

            view! {
                <g
                    transform=format!("translate({nx:.1},{ny:.1})")
                    style="cursor:pointer"
                    on:click=move |_| {
                        if let Some(cb) = on_node_click {
                            cb.run(node_id.clone());
                        }
                    }
                >
                    // Outer glow ring
                    <circle r=format!("{:.0}", NODE_R + 5.0) fill=glow_color stroke="none" />
                    // Main circle
                    <circle
                        r=format!("{NODE_R:.0}")
                        fill=fill_color
                        stroke=ring_color
                        stroke-width="2.5"
                    />
                    // Category icon (Lucide, centered in circle, white on category fill)
                    <g transform="translate(-8,-8)">
                        <Icon
                            icon=Signal::derive(move || cat_icon)
                            width="16"
                            height="16"
                            style="color:white"
                        />
                    </g>
                    // Primary label
                    <text
                        x="0" y=format!("{LABEL_OFFSET:.0}")
                        text-anchor="middle"
                        font-size="9"
                        font-weight="600"
                        fill="#374151"
                    >{label_text}</text>
                    // Sub-label
                    <text
                        x="0" y=format!("{SUBLABEL_OFFSET:.0}")
                        text-anchor="middle"
                        font-size="8"
                        fill="#9ca3af"
                    >{sublabel_text}</text>
                    {sev_badge}
                </g>
            }
            .into_any()
        })
        .collect();

    // ── Transform string (reactive) ───────────────────────────────────────────
    let transform = move || {
        format!(
            "translate({:.2},{:.2}) scale({:.4})",
            pan_x.get(),
            pan_y.get(),
            zoom.get()
        )
    };

    let svg_class = format!(
        "w-full h-auto select-none border border-border rounded-xl bg-card {}",
        class
    );

    let cursor_style = move || {
        if dragging.get() {
            "cursor:grabbing"
        } else {
            "cursor:grab"
        }
    };

    view! {
        <svg
            viewBox=vb
            class=svg_class
            style=cursor_style
            on:mousedown=move |e| {
                dragging.set(true);
                last_xy.set((e.client_x() as f64, e.client_y() as f64));
                e.prevent_default();
            }
            on:mousemove=move |e| {
                if !dragging.get() { return; }
                let (lx, ly) = last_xy.get();
                let z = zoom.get_untracked();
                pan_x.update(|v| *v += (e.client_x() as f64 - lx) / z);
                pan_y.update(|v| *v += (e.client_y() as f64 - ly) / z);
                last_xy.set((e.client_x() as f64, e.client_y() as f64));
            }
            on:mouseup=move |_| dragging.set(false)
            on:mouseleave=move |_| dragging.set(false)
            on:wheel=move |e| {
                e.prevent_default();
                let factor = if e.delta_y() < 0.0 { 1.12 } else { 0.90 };
                zoom.update(|z| *z = (*z * factor).clamp(0.15, 6.0));
            }
        >
            <defs>
                <marker
                    id="sg-arrow"
                    markerWidth="8"
                    markerHeight="6"
                    refX="7"
                    refY="3"
                    orient="auto"
                >
                    <polygon points="0 0, 8 3, 0 6" fill="#9ca3af" />
                </marker>
            </defs>
            <g transform=transform>
                {edge_views}
                {node_views}
            </g>
        </svg>
    }
}
