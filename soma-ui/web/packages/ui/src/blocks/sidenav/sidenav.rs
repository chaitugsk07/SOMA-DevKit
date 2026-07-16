use crate::components::data_display::avatar::Avatar;
use leptos::prelude::*;

#[component]
pub fn SidenavBlock() -> impl IntoView {
    let active = RwSignal::new("dashboard".to_string());

    let link_class = move |id: &str| {
        let base = "flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-colors cursor-pointer";
        if active.get() == id {
            format!(
                "{} border-l-[3px] border-primary text-primary bg-accent font-medium",
                base
            )
        } else {
            format!(
                "{} border-l-[3px] border-transparent text-muted-foreground hover:text-foreground hover:bg-muted",
                base
            )
        }
    };

    view! {
        <div class="w-64 min-h-[600px] bg-card border-e border-border flex flex-col">
            // Brand header
            <div class="p-4 border-b border-border">
                <span class="font-heading text-lg font-bold text-foreground">"Soma UI"</span>
            </div>

            // Nav groups
            <nav class="flex-1 p-3 space-y-6 overflow-y-auto">
                <div>
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-widest px-3 mb-1">"General"</p>
                    <div class="space-y-0.5">
                        <div class=move || link_class("dashboard") on:click=move |_| active.set("dashboard".to_string())>
                            <span>"Dashboard"</span>
                        </div>
                        <div class=move || link_class("analytics") on:click=move |_| active.set("analytics".to_string())>
                            <span>"Analytics"</span>
                        </div>
                        <div class=move || link_class("reports") on:click=move |_| active.set("reports".to_string())>
                            <span>"Reports"</span>
                        </div>
                    </div>
                </div>
                <div>
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-widest px-3 mb-1">"Manage"</p>
                    <div class="space-y-0.5">
                        <div class=move || link_class("users") on:click=move |_| active.set("users".to_string())>
                            <span>"Users"</span>
                        </div>
                        <div class=move || link_class("projects") on:click=move |_| active.set("projects".to_string())>
                            <span>"Projects"</span>
                        </div>
                        <div class=move || link_class("settings") on:click=move |_| active.set("settings".to_string())>
                            <span>"Settings"</span>
                        </div>
                    </div>
                </div>
            </nav>

            // User row
            <div class="p-3 border-t border-border">
                <div class="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent transition-colors cursor-pointer">
                    <Avatar fallback="JD".to_string() />
                    <div class="flex flex-col min-w-0">
                        <span class="text-sm font-medium text-foreground truncate">"Jane Doe"</span>
                        <span class="text-xs text-muted-foreground truncate">"jane@example.com"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
