use crate::icons::{icondata, Icon};
use crate::{
    AreaChart, AreaVariant, Avatar, AvatarSize, Badge, BadgeVariant, Button, ButtonSize,
    ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, ChartPoint, Command,
    CommandEmpty, CommandGroup, CommandItem, ConsoleShell, Table, TableBody, TableCell, TableHead,
    TableHeader, TableRow,
};
use leptos::prelude::*;
use std::sync::Arc;

#[component]
fn ConsoleNavItem(
    #[prop(into)] label: String,
    icon: icondata::Icon,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    let class = if active {
        "flex items-center gap-2.5 rounded-md border-s-2 border-primary bg-accent px-3 py-2 text-sm font-medium text-foreground"
    } else {
        "flex items-center gap-2.5 rounded-md border-s-2 border-transparent px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
    };

    view! {
        <a href="#" class=class aria-current=active.then_some("page")>
            <Icon icon=Signal::derive(move || icon) width="16" height="16" />
            {label}
        </a>
    }
}

#[component]
pub fn DashboardScreen() -> impl IntoView {
    let command_open = RwSignal::new(false);
    window_event_listener(leptos::ev::keydown, move |event| {
        if (event.ctrl_key() || event.meta_key()) && event.key().eq_ignore_ascii_case("k") {
            event.prevent_default();
            command_open.set(true);
        }
    });

    let metrics = vec![
        ("ACTIVE AGENTS", "24", "+3 this week", BadgeVariant::Success),
        ("WORKFLOW RUNS", "1,284", "+12.4%", BadgeVariant::Success),
        ("SUCCESS RATE", "98.7%", "+0.8%", BadgeVariant::Success),
        (
            "COMPUTE SPEND",
            "$8.4K",
            "72% budget",
            BadgeVariant::Outline,
        ),
    ];

    let chart_data = vec![
        ChartPoint {
            label: "Mon".into(),
            value: 128.0,
        },
        ChartPoint {
            label: "Tue".into(),
            value: 184.0,
        },
        ChartPoint {
            label: "Wed".into(),
            value: 156.0,
        },
        ChartPoint {
            label: "Thu".into(),
            value: 221.0,
        },
        ChartPoint {
            label: "Fri".into(),
            value: 206.0,
        },
        ChartPoint {
            label: "Sat".into(),
            value: 144.0,
        },
        ChartPoint {
            label: "Sun".into(),
            value: 178.0,
        },
    ];

    let activity = vec![
        ("support-triage", "Agent", "Maya Chen", "2m ago", "Running"),
        (
            "invoice-review",
            "Workflow",
            "Finance Ops",
            "8m ago",
            "Completed",
        ),
        (
            "customer-health",
            "Agent",
            "RevOps",
            "23m ago",
            "Needs review",
        ),
        (
            "sync-crm-contacts",
            "Workflow",
            "Data Platform",
            "1h ago",
            "Failed",
        ),
        (
            "forecast-refresh",
            "Workflow",
            "Planning",
            "2h ago",
            "Completed",
        ),
    ];

    let sidebar = Arc::new(move || {
        view! {
            <div class="flex flex-col gap-0.5 p-3">
                <p class="px-3 py-2 font-heading text-xs font-semibold uppercase tracking-widest text-muted-foreground">
                    "BUILD"
                </p>
                <ConsoleNavItem label="Overview" icon=icondata::LuLayoutDashboard active=true />
                <ConsoleNavItem label="Agents" icon=icondata::LuBot />
                <ConsoleNavItem label="Workflows" icon=icondata::LuGitBranch />
                <ConsoleNavItem label="Data" icon=icondata::LuDatabase />
                <ConsoleNavItem label="Runs" icon=icondata::LuPlay />

                <div class="my-2 border-t border-border" />
                <p class="px-3 py-2 font-heading text-xs font-semibold uppercase tracking-widest text-muted-foreground">
                    "OPERATE"
                </p>
                <ConsoleNavItem label="Observability" icon=icondata::LuActivity />
                <ConsoleNavItem label="Audit log" icon=icondata::LuClipboardList />
            </div>
        }
        .into_any()
    });

    let sidebar_footer = Arc::new(move || {
        view! {
            <div class="border-t border-border p-3">
                <div class="mb-2 flex items-center gap-2">
                    <span class="h-1.5 w-1.5 shrink-0 rounded-full bg-success" />
                    <span class="font-mono text-xs text-muted-foreground">"PRODUCTION · HEALTHY"</span>
                </div>
                <div class="flex items-center gap-2">
                    <Avatar fallback="AO".to_string() size=AvatarSize::Sm />
                    <div class="flex min-w-0 flex-col">
                        <span class="truncate text-xs font-medium text-foreground">"Alex Operator"</span>
                        <span class="truncate text-xs text-muted-foreground">"SOMA Platform"</span>
                    </div>
                </div>
            </div>
        }
        .into_any()
    });

    let topbar = Arc::new(move || {
        view! {
            <div class="hidden items-center gap-1.5 text-sm md:flex">
                <span class="text-muted-foreground">"Acme Operations"</span>
                <span class="text-muted-foreground">"/"</span>
                <span class="font-medium text-foreground">"Production"</span>
            </div>
            <span class="truncate text-sm font-medium text-foreground md:hidden">"Overview"</span>

            <button
                type="button"
                class="mx-auto hidden h-8 w-full max-w-xs items-center gap-2 rounded-md border border-input bg-card px-2.5 text-xs text-muted-foreground transition-colors hover:border-ring/50 hover:text-foreground sm:flex"
                on:click=move |_| command_open.set(true)
            >
                <Icon icon=Signal::derive(|| icondata::LuSearch) width="14" height="14" />
                <span class="truncate">"Search resources and commands"</span>
                <span class="ms-auto rounded border border-border bg-muted px-1.5 py-0.5 font-mono text-[10px]">"⌘K"</span>
            </button>

            <div class="ms-auto flex items-center gap-1">
                <Button
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::Icon
                    aria_label="Notifications".to_string()
                >
                    <Icon icon=Signal::derive(|| icondata::LuBell) width="16" height="16" />
                </Button>
                <Avatar fallback="AO".to_string() size=AvatarSize::Sm />
            </div>
        }
        .into_any()
    });

    view! {
        <ConsoleShell
            brand="SOMA Console"
            version="v0.2"
            sidebar=sidebar
            sidebar_footer=sidebar_footer
            topbar=topbar
        >
            <div class="p-4 md:p-6">
                <div class="mb-6 flex flex-col justify-between gap-3 sm:flex-row sm:items-end">
                    <div>
                        <div class="mb-2 flex items-center gap-2">
                            <Badge variant=BadgeVariant::Outline>"PRODUCTION"</Badge>
                            <span class="inline-flex items-center gap-1.5 text-xs text-success">
                                <span class="h-1.5 w-1.5 rounded-full bg-success" />
                                "All systems operational"
                            </span>
                        </div>
                        <h1 class="font-heading text-2xl font-bold tracking-tight text-foreground">"Operations overview"</h1>
                        <p class="mt-1 text-sm text-muted-foreground">
                            "Build agents, orchestrate workflows, and monitor production from one workspace."
                        </p>
                    </div>
                    <Button>
                        <Icon icon=Signal::derive(|| icondata::LuPlus) width="16" height="16" />
                        "New workflow"
                    </Button>
                </div>

                <div class="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
                    {metrics.into_iter().map(|(label, value, delta, variant)| view! {
                        <Card>
                            <CardContent class="p-4".to_string()>
                                <p class="mb-2 font-heading text-xs font-semibold uppercase tracking-widest text-muted-foreground">
                                    {label}
                                </p>
                                <p class="font-mono text-2xl font-bold tabular-nums text-foreground">{value}</p>
                                <div class="mt-2">
                                    <Badge variant=variant class="font-mono text-xs".to_string()>{delta}</Badge>
                                </div>
                            </CardContent>
                        </Card>
                    }).collect::<Vec<_>>()}
                </div>

                <div class="mb-6 grid grid-cols-1 gap-4 xl:grid-cols-[minmax(0,2fr)_minmax(16rem,1fr)]">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Execution volume · 7-day"</CardTitle>
                            <CardDescription>"Agent and workflow runs across the production environment"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <AreaChart data=chart_data variant=AreaVariant::Gradient class="w-full".to_string() />
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Operations"</CardTitle>
                            <CardDescription>"Live workload and governance signals"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-4".to_string()>
                            <div class="flex items-center justify-between border-b border-border pb-3">
                                <span class="text-sm text-muted-foreground">"Running now"</span>
                                <span class="font-mono text-sm font-semibold text-foreground">"18"</span>
                            </div>
                            <div class="flex items-center justify-between border-b border-border pb-3">
                                <span class="text-sm text-muted-foreground">"Awaiting review"</span>
                                <Badge variant=BadgeVariant::Secondary>"3"</Badge>
                            </div>
                            <div class="flex items-center justify-between border-b border-border pb-3">
                                <span class="text-sm text-muted-foreground">"Open incidents"</span>
                                <Badge variant=BadgeVariant::Destructive>"1"</Badge>
                            </div>
                            <div class="flex items-center justify-between">
                                <span class="text-sm text-muted-foreground">"Policy coverage"</span>
                                <span class="font-mono text-sm font-semibold text-success">"100%"</span>
                            </div>
                        </CardContent>
                    </Card>
                </div>

                <Card>
                    <CardHeader>
                        <CardTitle>"Recent activity"</CardTitle>
                        <CardDescription>"Latest production runs across agents and workflows"</CardDescription>
                    </CardHeader>
                    <CardContent class="p-0".to_string()>
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead>"RESOURCE"</TableHead>
                                    <TableHead>"TYPE"</TableHead>
                                    <TableHead>"OWNER"</TableHead>
                                    <TableHead>"LAST RUN"</TableHead>
                                    <TableHead>"STATUS"</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {activity.into_iter().map(|(name, kind, owner, time, status)| {
                                    let badge_variant = match status {
                                        "Completed" | "Running" => BadgeVariant::Success,
                                        "Needs review" => BadgeVariant::Secondary,
                                        _ => BadgeVariant::Destructive,
                                    };
                                    view! {
                                        <TableRow>
                                            <TableCell><span class="font-mono text-xs">{name}</span></TableCell>
                                            <TableCell><span class="text-xs text-muted-foreground">{kind}</span></TableCell>
                                            <TableCell><span class="text-xs">{owner}</span></TableCell>
                                            <TableCell><span class="text-xs text-muted-foreground">{time}</span></TableCell>
                                            <TableCell><Badge variant=badge_variant>{status}</Badge></TableCell>
                                        </TableRow>
                                    }
                                }).collect::<Vec<_>>()}
                            </TableBody>
                        </Table>
                    </CardContent>
                </Card>
            </div>
        </ConsoleShell>

        <Command open=command_open>
            <CommandGroup heading="Create">
                <CommandItem keywords="new agent create">"Create agent"</CommandItem>
                <CommandItem keywords="new workflow automation create">"Create workflow"</CommandItem>
                <CommandItem keywords="connect data source integration">"Connect data source"</CommandItem>
            </CommandGroup>
            <CommandGroup heading="Navigate">
                <CommandItem keywords="agents assistants">"Open agents"</CommandItem>
                <CommandItem keywords="runs jobs executions">"View runs"</CommandItem>
                <CommandItem keywords="audit security governance">"Open audit log"</CommandItem>
            </CommandGroup>
            <CommandEmpty>"No matching resources or commands."</CommandEmpty>
        </Command>
    }
}
