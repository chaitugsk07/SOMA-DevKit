use crate::{
    AreaChart, AreaVariant, Avatar, AvatarSize, Badge, BadgeVariant, Button, ButtonSize,
    ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, ChartPoint, Input,
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};
use leptos::prelude::*;

#[component]
pub fn DashboardScreen() -> impl IntoView {
    let sidebar_open = RwSignal::new(false);
    let search = RwSignal::new(String::new());

    let metrics = vec![
        ("ACTIVE PIPELINES", "1,284", "+12%", true),
        ("DATA PROCESSED", "94.2 TB", "+8.3%", true),
        ("QUERY LATENCY", "142 ms", "-5.1%", false),
        ("ERROR RATE", "0.03%", "-0.01%", false),
    ];

    let chart_data = vec![
        ChartPoint {
            label: "Mon".into(),
            value: 420.0,
        },
        ChartPoint {
            label: "Tue".into(),
            value: 680.0,
        },
        ChartPoint {
            label: "Wed".into(),
            value: 540.0,
        },
        ChartPoint {
            label: "Thu".into(),
            value: 810.0,
        },
        ChartPoint {
            label: "Fri".into(),
            value: 760.0,
        },
        ChartPoint {
            label: "Sat".into(),
            value: 390.0,
        },
        ChartPoint {
            label: "Sun".into(),
            value: 520.0,
        },
    ];

    let activity = vec![
        ("pipeline-alpha-7", "2m ago", "Active"),
        ("etl-transform-14", "8m ago", "Active"),
        ("report-daily-fin", "23m ago", "Pending"),
        ("sync-crm-contacts", "1h ago", "Failed"),
        ("ml-inference-v3", "2h ago", "Active"),
    ];

    view! {
        <div class="flex h-screen bg-background overflow-hidden">
            // Sidebar (desktop)
            <aside class="hidden md:flex flex-col w-60 bg-card border-e border-border h-screen sticky top-0 overflow-y-auto shrink-0">
                // Brand header
                <div class="flex items-center gap-2 px-4 h-14 border-b border-border shrink-0">
                    <svg width="20" height="20" viewBox="0 0 56 56" fill="none">
                        <polygon points="28,4 52,16 52,40 28,52 4,40 4,16" stroke="hsl(var(--primary))" stroke-width="2.5" fill="hsl(var(--primary) / 0.1)"/>
                    </svg>
                    <span class="font-heading font-bold text-sm text-foreground">"Foundry"</span>
                    <span class="ms-auto font-mono text-xs bg-primary/10 text-primary px-1.5 py-0.5 rounded">"v0.1"</span>
                </div>
                // Nav
                <nav class="flex flex-col gap-0.5 p-3 flex-1 overflow-y-auto">
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-widest px-3 py-2 font-heading">"WORKSPACE"</p>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-foreground bg-accent border-s-2 border-primary font-medium">
                        <span class="w-4 h-4 opacity-70" inner_html=r#"<svg viewBox="0 0 16 16" fill="currentColor"><path d="M2 2h5v5H2zm7 0h5v5H9zM2 9h5v5H2zm7 0h5v5H9z"/></svg>"# />
                        "Overview"
                    </a>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
                        <span class="w-4 h-4 opacity-70" inner_html=r#"<svg viewBox="0 0 16 16" fill="currentColor"><path d="M1 2.5A1.5 1.5 0 0 1 2.5 1h11A1.5 1.5 0 0 1 15 2.5v6.086a1.5 1.5 0 0 1-.44 1.06l-6 6a1.5 1.5 0 0 1-2.12 0l-6-6A1.5 1.5 0 0 1 0 8.586V4a1.5 1.5 0 0 1 1.5-1.5h1z"/></svg>"# />
                        "Pipelines"
                    </a>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
                        <span class="w-4 h-4 opacity-70" inner_html=r#"<svg viewBox="0 0 16 16" fill="currentColor"><path d="M1 2.5A1.5 1.5 0 0 1 2.5 1h3A1.5 1.5 0 0 1 7 2.5v3A1.5 1.5 0 0 1 5.5 7h-3A1.5 1.5 0 0 1 1 5.5v-3zm8 0A1.5 1.5 0 0 1 10.5 1h3A1.5 1.5 0 0 1 15 2.5v3A1.5 1.5 0 0 1 13.5 7h-3A1.5 1.5 0 0 1 9 5.5v-3zm-8 8A1.5 1.5 0 0 1 2.5 9h3A1.5 1.5 0 0 1 7 10.5v3A1.5 1.5 0 0 1 5.5 15h-3A1.5 1.5 0 0 1 1 13.5v-3zm8 0A1.5 1.5 0 0 1 10.5 9h3A1.5 1.5 0 0 1 15 10.5v3A1.5 1.5 0 0 1 13.5 15h-3A1.5 1.5 0 0 1 9 13.5v-3z"/></svg>"# />
                        "Datasets"
                    </a>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
                        <span class="w-4 h-4 opacity-70" inner_html=r#"<svg viewBox="0 0 16 16" fill="currentColor"><path d="M6 1h1v1.5l.5.5H9v.5a.5.5 0 0 1-.5.5H5a.5.5 0 0 1-.5-.5V3h1.5L6 2.5V1zM1 4h14v1H1V4zm1 2h12l-1 8H3L2 6z"/></svg>"# />
                        "Reports"
                    </a>
                    <div class="my-2 border-t border-border"></div>
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-widest px-3 py-2 font-heading">"MONITORING"</p>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
                        <span class="w-4 h-4 opacity-70" inner_html=r#"<svg viewBox="0 0 16 16" fill="currentColor"><path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/><path d="M7.5 5.5A.5.5 0 0 1 8 5h.5a.5.5 0 0 1 .5.5v3h1a.5.5 0 0 1 0 1H8a.5.5 0 0 1-.5-.5v-3.5z"/></svg>"# />
                        "Alerts"
                    </a>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
                        <span class="w-4 h-4 opacity-70" inner_html=r#"<svg viewBox="0 0 16 16" fill="currentColor"><path d="M.5 9.9a.5.5 0 0 1 .5.5v2.1h2.1a.5.5 0 0 1 0 1H.5a.5.5 0 0 1-.5-.5v-2.6a.5.5 0 0 1 .5-.5zm14 0a.5.5 0 0 1 .5.5v2.6a.5.5 0 0 1-.5.5h-2.6a.5.5 0 0 1 0-1h2.1V10.4a.5.5 0 0 1 .5-.5zM0 6a.5.5 0 0 1 .5-.5h15a.5.5 0 0 1 0 1H.5A.5.5 0 0 1 0 6z"/></svg>"# />
                        "Audit Log"
                    </a>
                </nav>
                // Bottom user row
                <div class="border-t border-border p-3">
                    <div class="flex items-center gap-2 mb-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-green-500 shrink-0"></span>
                        <span class="font-mono text-xs text-muted-foreground">"CONNECTED"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <Avatar fallback="AO".to_string() size=AvatarSize::Sm />
                        <div class="flex flex-col min-w-0">
                            <span class="text-xs font-medium text-foreground truncate">"Alex Operator"</span>
                            <span class="text-xs text-muted-foreground truncate">"alex@foundry.io"</span>
                        </div>
                    </div>
                </div>
            </aside>

            // Mobile sidebar overlay
            <Show when=move || sidebar_open.get()>
                <div class="fixed inset-0 z-50 flex md:hidden">
                    <div class="absolute inset-0 bg-black/50" on:click=move |_| sidebar_open.set(false)></div>
                    <aside class="relative z-10 flex flex-col w-60 bg-card h-screen overflow-y-auto">
                        <div class="flex items-center gap-2 px-4 h-14 border-b border-border shrink-0">
                            <span class="font-heading font-bold text-sm text-foreground">"Foundry"</span>
                            <button class="ms-auto text-muted-foreground" on:click=move |_| sidebar_open.set(false)>"✕"</button>
                        </div>
                        <nav class="flex flex-col gap-0.5 p-3 flex-1">
                            <a href="#" on:click=move |_| sidebar_open.set(false) class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-foreground bg-accent">"Overview"</a>
                            <a href="#" on:click=move |_| sidebar_open.set(false) class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground">"Pipelines"</a>
                            <a href="#" on:click=move |_| sidebar_open.set(false) class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground">"Datasets"</a>
                            <a href="#" on:click=move |_| sidebar_open.set(false) class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground">"Reports"</a>
                        </nav>
                        <div class="border-t border-border p-3">
                            <div class="flex items-center gap-2 mb-2">
                                <span class="w-1.5 h-1.5 rounded-full bg-green-500"></span>
                                <span class="font-mono text-xs text-muted-foreground">"CONNECTED"</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <Avatar fallback="AO".to_string() size=AvatarSize::Sm />
                                <span class="text-xs text-foreground">"Alex Operator"</span>
                            </div>
                        </div>
                    </aside>
                </div>
            </Show>

            // Main content area
            <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
                // Topbar
                <header class="flex items-center gap-3 px-4 h-14 border-b border-border bg-background shrink-0">
                    // Hamburger (mobile only)
                    <button class="flex md:hidden items-center justify-center w-8 h-8 text-muted-foreground hover:text-foreground"
                        on:click=move |_| sidebar_open.set(true)
                        inner_html=r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"/></svg>"#
                    />
                    // Breadcrumb
                    <div class="hidden md:flex items-center gap-1.5 text-sm">
                        <span class="text-muted-foreground">"Workspace"</span>
                        <span class="text-muted-foreground">"/"</span>
                        <span class="text-foreground font-medium">"Overview"</span>
                    </div>
                    <span class="md:hidden text-sm font-medium text-foreground">"Overview"</span>
                    // Search (center, grows)
                    <div class="flex-1 max-w-xs mx-auto hidden sm:block">
                        <div class="relative">
                            <Input value=search placeholder="Search pipelines… ⌘K".to_string() class="h-8 text-xs ps-8".to_string() />
                            <span class="absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" inner_html=r#"<svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor"><path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001q.044.06.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1 1 0 0 0-.115-.099zm-5.242 1.656a5.5 5.5 0 1 1 0-11 5.5 5.5 0 0 1 0 11"/></svg>"# />
                        </div>
                    </div>
                    // Actions (right side)
                    <div class="ms-auto flex items-center gap-2">
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon aria_label="Notifications".to_string()>
                            <span inner_html=r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 16a2 2 0 0 0 2-2H6a2 2 0 0 0 2 2M8 1.918l-.797.161A4 4 0 0 0 4 6c0 .628-.134 2.197-.459 3.742-.16.767-.376 1.566-.663 2.258h10.244c-.287-.692-.502-1.49-.663-2.258C12.134 8.197 12 6.628 12 6a4 4 0 0 0-3.203-3.92zM14.22 12c.223.447.481.801.78 1H1c.299-.199.557-.553.78-1C2.68 10.2 3 6.88 3 6a5 5 0 0 1 10 0c0 .88.32 4.2 1.22 6"/></svg>"# />
                        </Button>
                        <Avatar fallback="AO".to_string() size=AvatarSize::Sm />
                    </div>
                </header>

                // Page content (scrollable)
                <main class="flex-1 overflow-y-auto p-4 md:p-6">
                    // Metric cards
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
                        {metrics.into_iter().map(|(label, value, delta, positive)| {
                            let badge_variant = if positive { BadgeVariant::Success } else { BadgeVariant::Destructive };
                            view! {
                                <Card>
                                    <CardContent class="p-4".to_string()>
                                        <p class="text-xs font-semibold text-muted-foreground uppercase tracking-widest font-heading mb-2">{label}</p>
                                        <p class="text-2xl font-mono tabular-nums font-bold text-foreground">{value}</p>
                                        <div class="mt-2">
                                            <Badge variant=badge_variant class="text-xs font-mono".to_string()>{delta}</Badge>
                                        </div>
                                    </CardContent>
                                </Card>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    // Chart card
                    <Card class="mb-6".to_string()>
                        <CardHeader>
                            <CardTitle>"Throughput · 7-day"</CardTitle>
                            <CardDescription>"Records processed per day across all active pipelines"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <AreaChart data=chart_data variant=AreaVariant::Gradient class="w-full".to_string() />
                        </CardContent>
                    </Card>

                    // Activity table
                    <Card>
                        <CardHeader>
                            <CardTitle>"Recent Activity"</CardTitle>
                            <CardDescription>"Latest pipeline executions"</CardDescription>
                        </CardHeader>
                        <CardContent class="p-0".to_string()>
                            <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHead>"PIPELINE"</TableHead>
                                        <TableHead>"LAST RUN"</TableHead>
                                        <TableHead>"STATUS"</TableHead>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    {activity.into_iter().map(|(name, time, status)| {
                                        let badge_var = match status {
                                            "Active" => BadgeVariant::Success,
                                            "Pending" => BadgeVariant::Secondary,
                                            _ => BadgeVariant::Destructive,
                                        };
                                        view! {
                                            <TableRow>
                                                <TableCell><span class="font-mono text-xs">{name}</span></TableCell>
                                                <TableCell><span class="text-muted-foreground text-xs">{time}</span></TableCell>
                                                <TableCell><Badge variant=badge_var>{status}</Badge></TableCell>
                                            </TableRow>
                                        }
                                    }).collect::<Vec<_>>()}
                                </TableBody>
                            </Table>
                        </CardContent>
                    </Card>
                </main>
            </div>
        </div>
    }
}
