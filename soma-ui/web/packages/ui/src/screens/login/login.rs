use crate::{Button, ButtonVariant, Input, Label, Separator};
use leptos::prelude::*;

#[component]
pub fn LoginScreen() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    view! {
        <div class="min-h-screen bg-background flex flex-col md:grid md:grid-cols-2">
            // Left brand panel — hidden on mobile, shown on md+
            <div class="hidden md:flex flex-col bg-card border-e border-border p-12 relative overflow-hidden">
                // faint grid overlay
                <div class="absolute inset-0 opacity-5"
                     style="background-image: linear-gradient(hsl(var(--border)) 1px, transparent 1px), linear-gradient(90deg, hsl(var(--border)) 1px, transparent 1px); background-size: 32px 32px;" />
                <div class="relative flex flex-col flex-1 items-center justify-center gap-6 text-center">
                    // Logo mark — simple hexagon SVG
                    <svg width="56" height="56" viewBox="0 0 56 56" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <polygon points="28,4 52,16 52,40 28,52 4,40 4,16" stroke="hsl(var(--primary))" stroke-width="2" fill="hsl(var(--primary) / 0.08)"/>
                        <polygon points="28,14 42,21 42,35 28,42 14,35 14,21" stroke="hsl(var(--primary))" stroke-width="1.5" fill="none" opacity="0.5"/>
                    </svg>
                    <div>
                        <p class="font-heading text-2xl font-bold text-foreground tracking-tight">"Foundry"</p>
                        <p class="text-sm text-muted-foreground mt-1">"Operational intelligence platform"</p>
                    </div>
                </div>
                // Status footer
                <div class="relative flex items-center gap-2 mt-auto">
                    <span class="w-1.5 h-1.5 rounded-full bg-green-500 inline-block"></span>
                    <span class="font-mono text-xs text-muted-foreground">"SYSTEM ONLINE · v0.1.0"</span>
                </div>
            </div>
            // Mobile header (slim, shown only on mobile)
            <div class="flex md:hidden items-center justify-between px-6 py-4 bg-card border-b border-border">
                <span class="font-heading font-bold text-foreground">"Foundry"</span>
                <span class="flex items-center gap-1.5 font-mono text-xs text-muted-foreground">
                    <span class="w-1.5 h-1.5 rounded-full bg-green-500 inline-block"></span>
                    "ONLINE"
                </span>
            </div>
            // Right auth panel
            <div class="flex flex-1 items-center justify-center p-6 md:p-12">
                <div class="w-full max-w-sm flex flex-col gap-6">
                    <div>
                        <h1 class="font-heading text-2xl font-bold text-foreground">"Sign in"</h1>
                        <p class="text-sm text-muted-foreground mt-1">"Enter your credentials to continue"</p>
                    </div>
                    <div class="flex flex-col gap-4">
                        <div class="flex flex-col gap-1.5">
                            <Label for_id="email".to_string()>"EMAIL"</Label>
                            <Input value=email placeholder="operator@foundry.io".to_string() input_type="email".to_string() />
                        </div>
                        <div class="flex flex-col gap-1.5">
                            <Label for_id="password".to_string()>"PASSWORD"</Label>
                            <Input value=password placeholder="••••••••".to_string() input_type="password".to_string() />
                        </div>
                        <Button class="w-full".to_string()>"Sign in"</Button>
                    </div>
                    <div class="relative flex items-center gap-3">
                        <Separator />
                        <span class="absolute left-1/2 -translate-x-1/2 bg-background px-2 text-xs text-muted-foreground whitespace-nowrap">"or"</span>
                    </div>
                    <Button variant=ButtonVariant::Outline class="w-full".to_string()>"Continue with SSO"</Button>
                    <p class="text-center text-xs text-muted-foreground">
                        "Need access? "
                        <a href="#" class="text-primary hover:underline">"Contact your administrator"</a>
                    </p>
                </div>
            </div>
        </div>
    }
}
