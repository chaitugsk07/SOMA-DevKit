use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn TooltipPage() -> impl IntoView {
    let tooltip_text = RwSignal::new("This is a tooltip".to_string());
    let trigger_label = RwSignal::new("Hover me".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Tooltip".to_string())
            subtitle=Signal::derive(move || "A small informational label that appears on hover. Pure CSS — no JavaScript.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Tooltip>
                        <TooltipTrigger>
                            <Button>{trigger_label.get()}</Button>
                        </TooltipTrigger>
                        <TooltipContent>
                            {tooltip_text.get()}
                        </TooltipContent>
                    </Tooltip>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Tooltip text">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| tooltip_text.set(event_target_value(&e))
                    >
                        <option value="This is a tooltip" selected>"This is a tooltip"</option>
                        <option value="Add to favourites">"Add to favourites"</option>
                        <option value="Click to expand">"Click to expand"</option>
                        <option value="Keyboard shortcut: ⌘K">"Keyboard shortcut: ⌘K"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Trigger label">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| trigger_label.set(event_target_value(&e))
                    >
                        <option value="Hover me" selected>"Hover me"</option>
                        <option value="Save">"Save"</option>
                        <option value="Delete">"Delete"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
