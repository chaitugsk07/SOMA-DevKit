use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Avatar, AvatarSize};

fn parse_size(s: &str) -> AvatarSize {
    match s {
        "Sm" => AvatarSize::Sm,
        "Lg" => AvatarSize::Lg,
        _ => AvatarSize::Md,
    }
}

#[component]
pub fn AvatarPage() -> impl IntoView {
    let size = RwSignal::new(AvatarSize::Md);
    let fallback = RwSignal::new("AB".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Avatar".to_string())
            subtitle=Signal::derive(move || "User avatar with image or initials fallback.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Avatar size=size.get() fallback=fallback.get() />
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Size">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| size.set(parse_size(&event_target_value(&e)))
                    >
                        <option value="Sm">"Sm"</option>
                        <option value="Md" selected>"Md"</option>
                        <option value="Lg">"Lg"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Initials">
                    <input
                        type="text"
                        maxlength="2"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-20"
                        prop:value=move || fallback.get()
                        on:input=move |e| fallback.set(event_target_value(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="flex flex-wrap items-center gap-4">
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size=AvatarSize::Sm fallback="SM".to_string() />
                        <span class="text-xs text-muted-foreground">"Sm"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size=AvatarSize::Md fallback="MD".to_string() />
                        <span class="text-xs text-muted-foreground">"Md"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size=AvatarSize::Lg fallback="LG".to_string() />
                        <span class="text-xs text-muted-foreground">"Lg"</span>
                    </div>
                </div>
            </div>
        </PageShell>
    }
}
