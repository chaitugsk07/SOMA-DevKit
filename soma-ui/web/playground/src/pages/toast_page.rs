use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{use_toast, Button, ButtonVariant, ToastProvider, ToastVariant, Toaster};

fn parse_toast_variant(s: &str) -> ToastVariant {
    match s {
        "Success" => ToastVariant::Success,
        "Destructive" => ToastVariant::Destructive,
        "Warning" => ToastVariant::Warning,
        "Info" => ToastVariant::Info,
        _ => ToastVariant::Default,
    }
}

#[component]
fn ToastDemo() -> impl IntoView {
    let toast = use_toast();
    let variant = RwSignal::new(ToastVariant::Default);

    view! {
        <div class="flex flex-col items-center gap-4 w-full max-w-xs">
            // Preview trigger
            <Button
                on:click=move |_| toast.show("Notification", None, variant.get(), 4000)
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
                            on:change=move |e| variant.set(parse_toast_variant(&event_target_value(&e)))
                        >
                            <option value="Default">"Default"</option>
                            <option value="Success">"Success"</option>
                            <option value="Destructive">"Destructive"</option>
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
pub fn ToastPage() -> impl IntoView {
    view! {
        <ToastProvider>
            <div class="max-w-3xl space-y-8">
                <div>
                    <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">"Toast"</h1>
                    <p class="text-sm text-muted-foreground mt-1">"Imperative toast notifications rendered in a portal. Auto-dismiss after 4 s. Wrap your app with "<code class="font-mono text-xs">"<ToastProvider>"</code>" and place "<code class="font-mono text-xs">"<Toaster />"</code>" as a sibling to your router."</p>
                </div>
                <PreviewPanel>
                    <ToastDemo />
                </PreviewPanel>
                // All Variants
                <div class="bg-card border border-border rounded-md p-6">
                    <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                    <AllToastVariants />
                </div>
            </div>
            <Toaster />
        </ToastProvider>
    }
}

#[component]
fn AllToastVariants() -> impl IntoView {
    let toast = use_toast();
    view! {
        <div class="flex flex-wrap gap-3">
            <Button on:click=move |_| toast.show("Notification", None, ToastVariant::Default, 4000)>
                "Default"
            </Button>
            <Button
                variant=ButtonVariant::Secondary
                on:click=move |_| toast.show("Success", Some("Operation completed successfully.".to_string()), ToastVariant::Success, 4000)
            >
                "Success"
            </Button>
            <Button
                variant=ButtonVariant::Destructive
                on:click=move |_| toast.show("Error", Some("Something went wrong. Please try again.".to_string()), ToastVariant::Destructive, 4000)
            >
                "Error"
            </Button>
            <Button
                variant=ButtonVariant::Outline
                on:click=move |_| toast.show("Info", Some("Your changes have been saved.".to_string()), ToastVariant::Info, 4000)
            >
                "With description"
            </Button>
        </div>
    }
}
