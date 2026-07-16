use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{HoverCard, HoverCardContent, HoverCardTrigger};

#[component]
pub fn HoverCardPage() -> impl IntoView {
    let username = RwSignal::new("@username".to_string());
    let show_stats = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Hover Card".to_string())
            subtitle=Signal::derive(move || "A rich preview card that appears on hover. Pure CSS — no JavaScript.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex items-center justify-center min-h-72">
                {move || {
                    let name = username.get();
                    let display = name.clone();
                    view! {
                        <HoverCard>
                            <HoverCardTrigger>
                                <span class="text-primary underline underline-offset-4 cursor-pointer font-medium">{display.clone()}</span>
                            </HoverCardTrigger>
                            <HoverCardContent>
                                <div class="space-y-3">
                                    <div class="flex items-center gap-3">
                                        <div class="w-10 h-10 rounded-full bg-zinc-700 flex items-center justify-center">
                                            <span class="text-sm font-bold text-zinc-300">"U"</span>
                                        </div>
                                        <div>
                                            <p class="text-sm font-semibold text-zinc-100">"Username"</p>
                                            <p class="text-xs text-zinc-400">{name}</p>
                                        </div>
                                    </div>
                                    <p class="text-sm text-zinc-300">"Building open-source tools for the web. Leptos enthusiast."</p>
                                    {move || show_stats.get().then(|| view! {
                                        <div class="flex items-center gap-4 text-xs text-zinc-400">
                                            <span><span class="font-semibold text-zinc-100">"1.2k"</span>" followers"</span>
                                            <span><span class="font-semibold text-zinc-100">"340"</span>" following"</span>
                                        </div>
                                    })}
                                </div>
                            </HoverCardContent>
                        </HoverCard>
                    }
                }}
            </div>

            <ControlsPanel>
                <ControlRow label="Username">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| username.set(event_target_value(&e))
                    >
                        <option value="@username" selected>"@username"</option>
                        <option value="@leptos_dev">"@leptos_dev"</option>
                        <option value="@rustacean">"@rustacean"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Show follower stats">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| show_stats.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
