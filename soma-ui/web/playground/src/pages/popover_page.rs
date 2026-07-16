use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, Popover, PopoverContent, PopoverTrigger};

#[component]
pub fn PopoverPage() -> impl IntoView {
    let trigger_label = RwSignal::new("Open Popover".to_string());
    let show_footer = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Popover".to_string())
            subtitle=Signal::derive(move || "An anchored floating panel that opens on click. Closes on outside click.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Popover>
                        <PopoverTrigger>
                            <Button>{trigger_label.get()}</Button>
                        </PopoverTrigger>
                        <PopoverContent>
                            <div class="space-y-3 w-64">
                                <div>
                                    <p class="text-sm font-medium text-zinc-100">"Quick Settings"</p>
                                    <p class="text-xs text-zinc-400 mt-0.5">"Adjust your preferences."</p>
                                </div>
                                <div class="space-y-1">
                                    <label class="text-xs text-zinc-400">"Display name"</label>
                                    <div class="h-8 rounded border border-zinc-700 bg-zinc-800 px-2 flex items-center">
                                        <span class="text-sm text-zinc-300">"John Doe"</span>
                                    </div>
                                </div>
                                {move || show_footer.get().then(|| view! {
                                    <Button class="w-full".to_string()>"Save changes"</Button>
                                })}
                            </div>
                        </PopoverContent>
                    </Popover>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Trigger label">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| trigger_label.set(event_target_value(&e))
                    >
                        <option value="Open Popover" selected>"Open Popover"</option>
                        <option value="Settings">"Settings"</option>
                        <option value="Edit Profile">"Edit Profile"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Show footer button">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| show_footer.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
