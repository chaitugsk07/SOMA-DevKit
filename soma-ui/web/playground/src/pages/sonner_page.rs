use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{use_sonner, Button, ButtonVariant, SonnerToaster, SonnerVariant};

fn parse_sonner_variant(s: &str) -> SonnerVariant {
    match s {
        "Success" => SonnerVariant::Success,
        "Error" => SonnerVariant::Error,
        "Warning" => SonnerVariant::Warning,
        "Info" => SonnerVariant::Info,
        _ => SonnerVariant::Default,
    }
}

#[component]
fn SonnerDemo() -> impl IntoView {
    let sonner = use_sonner();
    let variant = RwSignal::new(SonnerVariant::Default);

    let message_for = |v: SonnerVariant| match v {
        SonnerVariant::Default => "Notification sent",
        SonnerVariant::Success => "Changes saved successfully",
        SonnerVariant::Error => "Failed to save changes",
        SonnerVariant::Warning => "Storage is almost full",
        SonnerVariant::Info => "New update available",
    };

    view! {
        <div class="flex flex-col items-center gap-4 w-full max-w-xs">
            // Preview trigger
            <Button
                variant=ButtonVariant::Default
                on:click=move |_| sonner.show(message_for(variant.get()), variant.get(), 4000)
            >
                "Show toast"
            </Button>

            // Inline controls
            <div class="w-full bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
                <div class="space-y-0">
                    <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3 border-b border-border last:border-0">
                        <span class="text-sm text-muted-foreground">"Variant"</span>
                        <select
                            class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                            on:change=move |e| variant.set(parse_sonner_variant(&event_target_value(&e)))
                        >
                            <option value="Default">"Default"</option>
                            <option value="Success">"Success"</option>
                            <option value="Error">"Error"</option>
                            <option value="Warning">"Warning"</option>
                            <option value="Info">"Info"</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SonnerPage() -> impl IntoView {
    view! {
        <SonnerToaster>
            <div class="max-w-3xl space-y-8">
                <div>
                    <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">"Sonner"</h1>
                    <p class="text-sm text-muted-foreground mt-1">"Compact Sonner-style toast rows rendered in a portal. Auto-dismiss after 4 s. Mount "<code class="font-mono text-xs">"<SonnerToaster>"</code>" once near your app root."</p>
                </div>
                <PreviewPanel>
                    <SonnerDemo />
                </PreviewPanel>
                // All Variants
                <div class="bg-card border border-border rounded-md p-6">
                    <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                    <AllSonnerVariants />
                </div>
            </div>
        </SonnerToaster>
    }
}

#[component]
fn AllSonnerVariants() -> impl IntoView {
    let sonner = use_sonner();
    view! {
        <div class="flex flex-wrap gap-3">
            <Button on:click=move |_| sonner.show("Notification sent", SonnerVariant::Default, 4000)>
                "Default"
            </Button>
            <Button
                variant=ButtonVariant::Secondary
                on:click=move |_| sonner.show("Changes saved successfully", SonnerVariant::Success, 4000)
            >
                "Success"
            </Button>
            <Button
                variant=ButtonVariant::Destructive
                on:click=move |_| sonner.show("Failed to save changes", SonnerVariant::Error, 4000)
            >
                "Error"
            </Button>
            <Button
                variant=ButtonVariant::Outline
                on:click=move |_| sonner.show("Storage is almost full", SonnerVariant::Warning, 4000)
            >
                "Warning"
            </Button>
            <Button
                variant=ButtonVariant::Outline
                on:click=move |_| sonner.show("New update available", SonnerVariant::Info, 4000)
            >
                "Info"
            </Button>
        </div>
    }
}
