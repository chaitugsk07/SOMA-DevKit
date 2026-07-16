use crate::{
    Avatar, AvatarSize, Button, Card, CardContent, CardDescription, CardHeader, CardTitle,
    Checkbox, Input, Label, Separator, Switch,
};
use leptos::prelude::*;

#[component]
pub fn SettingsScreen() -> impl IntoView {
    let sidebar_open = RwSignal::new(false);
    let display_name = RwSignal::new("Alex Operator".to_string());
    let email_val = RwSignal::new("alex@foundry.io".to_string());
    let notifications = RwSignal::new(true);
    let digest = RwSignal::new(false);
    let two_fa = RwSignal::new(true);
    let api_logging = RwSignal::new(true);

    view! {
        <div class="flex h-screen bg-background overflow-hidden">
            // Sidebar (desktop) — simplified shell
            <aside class="hidden md:flex flex-col w-60 bg-card border-e border-border h-screen sticky top-0 overflow-y-auto shrink-0">
                <div class="flex items-center gap-2 px-4 h-14 border-b border-border shrink-0">
                    <svg width="20" height="20" viewBox="0 0 56 56" fill="none">
                        <polygon points="28,4 52,16 52,40 28,52 4,40 4,16" stroke="hsl(var(--primary))" stroke-width="2.5" fill="hsl(var(--primary) / 0.1)"/>
                    </svg>
                    <span class="font-heading font-bold text-sm text-foreground">"Foundry"</span>
                </div>
                <nav class="flex flex-col gap-0.5 p-3 flex-1">
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">"Overview"</a>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">"Pipelines"</a>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">"Datasets"</a>
                    <div class="my-2 border-t border-border"></div>
                    <a href="#" class="flex items-center gap-2.5 px-3 py-2 rounded-md text-sm text-foreground bg-accent border-s-2 border-primary font-medium">"Settings"</a>
                </nav>
                <div class="border-t border-border p-3">
                    <div class="flex items-center gap-2 mb-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-green-500"></span>
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

            // Mobile overlay
            <Show when=move || sidebar_open.get()>
                <div class="fixed inset-0 z-50 flex md:hidden">
                    <div class="absolute inset-0 bg-black/50" on:click=move |_| sidebar_open.set(false)></div>
                    <aside class="relative z-10 flex flex-col w-60 bg-card h-screen p-4">
                        <button class="self-end text-muted-foreground mb-4" on:click=move |_| sidebar_open.set(false)>"✕"</button>
                        <a href="#" class="py-2 text-sm text-muted-foreground">"Overview"</a>
                        <a href="#" class="py-2 text-sm text-foreground font-medium">"Settings"</a>
                    </aside>
                </div>
            </Show>

            <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
                // Topbar
                <header class="flex items-center gap-3 px-4 h-14 border-b border-border bg-background shrink-0">
                    <button class="flex md:hidden items-center justify-center w-8 h-8 text-muted-foreground"
                        on:click=move |_| sidebar_open.set(true)
                        inner_html=r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"/></svg>"#
                    />
                    <div class="flex items-center gap-1.5 text-sm">
                        <span class="text-muted-foreground">"Workspace"</span>
                        <span class="text-muted-foreground">"/"</span>
                        <span class="text-foreground font-medium">"Settings"</span>
                    </div>
                    <div class="ms-auto">
                        <Avatar fallback="AO".to_string() size=AvatarSize::Sm />
                    </div>
                </header>

                // Settings content
                <main class="flex-1 overflow-y-auto p-4 md:p-6 max-w-2xl">
                    <h1 class="font-heading text-xl font-bold text-foreground mb-6">"Settings"</h1>

                    // Profile section
                    <Card class="mb-4".to_string()>
                        <CardHeader>
                            <CardTitle>"Profile"</CardTitle>
                            <CardDescription>"Update your display name and contact information."</CardDescription>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-4".to_string()>
                            <div class="flex flex-col gap-1.5">
                                <Label for_id="display-name".to_string()>"DISPLAY NAME"</Label>
                                <Input value=display_name />
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <Label for_id="email-settings".to_string()>"EMAIL ADDRESS"</Label>
                                <Input value=email_val input_type="email".to_string() />
                            </div>
                        </CardContent>
                    </Card>

                    // Preferences section
                    <Card class="mb-4".to_string()>
                        <CardHeader>
                            <CardTitle>"Preferences"</CardTitle>
                            <CardDescription>"Manage notification and digest settings."</CardDescription>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-4".to_string()>
                            <div class="flex items-center justify-between">
                                <div>
                                    <p class="text-sm font-medium text-foreground">"Pipeline notifications"</p>
                                    <p class="text-xs text-muted-foreground">"Receive alerts when pipeline status changes"</p>
                                </div>
                                <Switch checked=notifications />
                            </div>
                            <Separator />
                            <div class="flex items-center justify-between">
                                <div>
                                    <p class="text-sm font-medium text-foreground">"Daily digest email"</p>
                                    <p class="text-xs text-muted-foreground">"Summary of all activity delivered at 08:00 UTC"</p>
                                </div>
                                <Switch checked=digest />
                            </div>
                        </CardContent>
                    </Card>

                    // Security section
                    <Card class="mb-6".to_string()>
                        <CardHeader>
                            <CardTitle>"Security"</CardTitle>
                            <CardDescription>"Control authentication and audit settings."</CardDescription>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-4".to_string()>
                            <div class="flex items-center gap-3">
                                <Checkbox checked=two_fa />
                                <div>
                                    <p class="text-sm font-medium text-foreground">"Two-factor authentication"</p>
                                    <p class="text-xs text-muted-foreground">"Require TOTP or hardware key on sign-in"</p>
                                </div>
                            </div>
                            <Separator />
                            <div class="flex items-center gap-3">
                                <Checkbox checked=api_logging />
                                <div>
                                    <p class="text-sm font-medium text-foreground">"API access logging"</p>
                                    <p class="text-xs text-muted-foreground">"Log all API calls to the audit trail"</p>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <Button>"Save changes"</Button>
                </main>
            </div>
        </div>
    }
}
