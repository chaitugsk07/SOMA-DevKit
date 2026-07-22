use crate::components::data_display::severity_badge::Severity;
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

/// A node in an inline attack-path chain.
#[derive(Clone, Debug)]
pub struct ChainNode {
    pub label: String,
    pub sublabel: String,
    pub category: String,
    pub severity: Option<Severity>,
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

/// Compact horizontal attack-path chain.
///
/// Renders node pills connected by arrow labels — suitable for embedding
/// inside an `IssueCard` or detail panel. Uses the same color vocabulary as
/// `SecurityGraph` but without pan/zoom (pure HTML layout, scrolls horizontally).
#[component]
pub fn AttackPathChain(
    #[prop(default = vec![])] nodes: Vec<ChainNode>,
    /// Arrow labels between nodes. Length should be `nodes.len() - 1`.
    /// If shorter, remaining arrows render without a label.
    #[prop(default = vec![])]
    edge_labels: Vec<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let wrapper = format!("flex items-start overflow-x-auto gap-0 py-2 {}", class);

    let chain: Vec<_> = nodes
        .into_iter()
        .enumerate()
        .map(|(idx, node)| {
            let ring = node
                .severity
                .map(|s| s.hex())
                .unwrap_or_else(|| category_color(&node.category));
            let cat_icon = category_icon(&node.category);

            let edge_label = if idx == 0 {
                None
            } else {
                edge_labels.get(idx - 1).cloned()
            };

            let sev_badge = node.severity.map(|sev| {
                view! {
                    <span
                        class="mt-0.5 inline-block rounded px-1.5 py-0.5 text-[9px] font-semibold leading-none"
                        style=sev.badge_style()
                    >{sev.label()}</span>
                }
            });

            let arrow = edge_label.map(|lbl| {
                view! {
                    <div class="flex flex-col items-center justify-center shrink-0 px-1 self-center">
                        <span class="text-[9px] text-muted-foreground whitespace-nowrap mb-0.5">{lbl}</span>
                        <svg width="24" height="10" viewBox="0 0 24 10" aria-hidden="true">
                            <line x1="0" y1="5" x2="18" y2="5" stroke="#9ca3af" stroke-width="1.5" marker-end="url(#apc-arrow)" />
                            <defs>
                                <marker id="apc-arrow" markerWidth="6" markerHeight="5" refX="5" refY="2.5" orient="auto">
                                    <polygon points="0 0, 6 2.5, 0 5" fill="#9ca3af" />
                                </marker>
                            </defs>
                        </svg>
                    </div>
                }
            });

            view! {
                <>
                    {arrow}
                    <div class="flex flex-col items-center gap-1 shrink-0 min-w-[72px] max-w-[100px]">
                        // Circle node
                        <div
                            class="w-10 h-10 rounded-full flex items-center justify-center shrink-0"
                            style=format!(
                                "background-color:{}20;border:2px solid {};color:white",
                                ring, ring
                            )
                        >
                            <Icon
                                icon=Signal::derive(move || cat_icon)
                                width="18"
                                height="18"
                            />
                        </div>
                        // Labels
                        <div class="flex flex-col items-center text-center">
                            <span class="text-[10px] font-semibold text-foreground leading-tight truncate w-full text-center">{node.label}</span>
                            <span class="text-[9px] text-muted-foreground leading-tight truncate w-full text-center">{node.sublabel}</span>
                            {sev_badge}
                        </div>
                    </div>
                </>
            }
        })
        .collect();

    view! {
        <div class=wrapper>
            {chain}
        </div>
    }
}
