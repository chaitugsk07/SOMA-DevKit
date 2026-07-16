use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[component]
pub fn TabsPage() -> impl IntoView {
    let active_segment = RwSignal::new("account".to_string());
    let active_pill = RwSignal::new("account".to_string());
    let active_underline = RwSignal::new("account".to_string());

    view! {
        <PageShell
            title=Signal::derive(|| "Tabs".to_string())
            subtitle=Signal::derive(|| "A tabbed interface — one active panel at a time, driven by a shared signal.".to_string())
        >
            <PreviewPanel>
                // Segment (default)
                <div class="w-full max-w-md mb-8">
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">"Segment"</p>
                    <Tabs value=active_segment class="w-full".to_string()>
                        <TabsList>
                            <TabsTrigger value="account".to_string()>"Account"</TabsTrigger>
                            <TabsTrigger value="password".to_string()>"Password"</TabsTrigger>
                            <TabsTrigger value="settings".to_string()>"Settings"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="account".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Manage your account details and preferences here."
                            </div>
                        </TabsContent>
                        <TabsContent value="password".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Change your password and two-factor authentication settings."
                            </div>
                        </TabsContent>
                        <TabsContent value="settings".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Notification preferences, theme, and regional settings."
                            </div>
                        </TabsContent>
                    </Tabs>
                </div>

                // Pill — active = bg-contrast text-contrast-foreground (strong near-black fill)
                <div class="w-full max-w-md mb-8">
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">"Pill"</p>
                    <Tabs value=active_pill class="w-full".to_string()>
                        <TabsList variant=TabsVariant::Pill>
                            <TabsTrigger value="account".to_string()>"Account"</TabsTrigger>
                            <TabsTrigger value="password".to_string()>"Password"</TabsTrigger>
                            <TabsTrigger value="settings".to_string()>"Settings"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="account".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Manage your account details and preferences here."
                            </div>
                        </TabsContent>
                        <TabsContent value="password".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Change your password and two-factor authentication settings."
                            </div>
                        </TabsContent>
                        <TabsContent value="settings".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Notification preferences, theme, and regional settings."
                            </div>
                        </TabsContent>
                    </Tabs>
                </div>

                // Underline
                <div class="w-full max-w-md">
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">"Underline"</p>
                    <Tabs value=active_underline class="w-full".to_string()>
                        <TabsList variant=TabsVariant::Underline>
                            <TabsTrigger value="account".to_string()>"Account"</TabsTrigger>
                            <TabsTrigger value="password".to_string()>"Password"</TabsTrigger>
                            <TabsTrigger value="settings".to_string()>"Settings"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="account".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Manage your account details and preferences here."
                            </div>
                        </TabsContent>
                        <TabsContent value="password".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Change your password and two-factor authentication settings."
                            </div>
                        </TabsContent>
                        <TabsContent value="settings".to_string()>
                            <div class="rounded-md border border-border p-4 text-sm text-foreground">
                                "Notification preferences, theme, and regional settings."
                            </div>
                        </TabsContent>
                    </Tabs>
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Active (all variants)">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            let v = event_target_value(&e);
                            active_segment.set(v.clone());
                            active_pill.set(v.clone());
                            active_underline.set(v);
                        }
                    >
                        <option value="account" selected>"Account"</option>
                        <option value="password">"Password"</option>
                        <option value="settings">"Settings"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
