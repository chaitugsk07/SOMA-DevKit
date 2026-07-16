use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, Command, CommandEmpty, CommandGroup, CommandItem};

#[component]
pub fn CommandPage() -> impl IntoView {
    let open = RwSignal::new(false);
    let last_action = RwSignal::new(String::new());

    // Ctrl/⌘+K opens the palette
    window_event_listener(leptos::ev::keydown, move |e| {
        if (e.ctrl_key() || e.meta_key()) && e.key() == "k" {
            e.prevent_default();
            open.set(true);
        }
    });

    view! {
        <PageShell
            title=Signal::derive(|| "Command".to_string())
            subtitle=Signal::derive(|| "Command palette with search filtering. Open with the button or Ctrl/⌘ K.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-72">
                <Button on:click=move |_| open.set(true)>
                    "Open Command Palette"
                    <span class="ms-2 text-xs opacity-60">"⌘K"</span>
                </Button>

                <Show when=move || !last_action.get().is_empty()>
                    <p class="text-sm text-muted-foreground">
                        "Last action: "
                        <span class="text-foreground font-mono">{move || last_action.get()}</span>
                    </p>
                </Show>
            </div>

            <Command open=open>
                <CommandGroup heading="Navigation">
                    <CommandItem
                        keywords="go home"
                        on_select=Callback::new(move |_| last_action.set("Go Home".to_string()))
                    >
                        "Go Home"
                    </CommandItem>
                    <CommandItem
                        keywords="open settings"
                        on_select=Callback::new(move |_| last_action.set("Open Settings".to_string()))
                    >
                        "Open Settings"
                    </CommandItem>
                </CommandGroup>
                <CommandGroup heading="Actions">
                    <CommandItem
                        keywords="new document create"
                        on_select=Callback::new(move |_| last_action.set("New Document".to_string()))
                    >
                        "New Document"
                    </CommandItem>
                    <CommandItem
                        keywords="export download"
                        on_select=Callback::new(move |_| last_action.set("Export".to_string()))
                    >
                        "Export"
                    </CommandItem>
                    <CommandItem
                        keywords="delete remove"
                        on_select=Callback::new(move |_| last_action.set("Delete".to_string()))
                    >
                        "Delete"
                    </CommandItem>
                </CommandGroup>
                <CommandEmpty>"No matching commands."</CommandEmpty>
            </Command>
        </PageShell>
    }
}
