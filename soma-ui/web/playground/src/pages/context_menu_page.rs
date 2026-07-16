use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuSeparator, ContextMenuTrigger,
};

#[component]
pub fn ContextMenuPage() -> impl IntoView {
    let show_separator = RwSignal::new(true);
    let last_clicked = RwSignal::new("None".to_string());
    let area_label = RwSignal::new("Right-click me".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Context Menu".to_string())
            subtitle=Signal::derive(move || "A menu triggered by right-click, positioned at the cursor. Closes on outside click or item selection.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-72">
                {move || {
                    let sep = show_separator.get();
                    view! {
                        <ContextMenu>
                            <ContextMenuTrigger>
                                <div class="flex items-center justify-center min-h-32 w-80 rounded-md border-2 border-dashed border-zinc-600 text-sm text-zinc-400 select-none cursor-default">
                                    {area_label.get()}
                                </div>
                            </ContextMenuTrigger>
                            <ContextMenuContent>
                                <ContextMenuItem on_click=Callback::new(move |_| last_clicked.set("New File".to_string()))>"New File"</ContextMenuItem>
                                <ContextMenuItem on_click=Callback::new(move |_| last_clicked.set("New Folder".to_string()))>"New Folder"</ContextMenuItem>
                                {sep.then(|| view! { <ContextMenuSeparator /> })}
                                <ContextMenuItem on_click=Callback::new(move |_| last_clicked.set("Copy".to_string()))>"Copy"</ContextMenuItem>
                                <ContextMenuItem on_click=Callback::new(move |_| last_clicked.set("Paste".to_string()))>"Paste"</ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenu>
                    }
                }}
                <p class="text-xs text-muted-foreground">"Last selected: " {move || last_clicked.get()}</p>
            </div>

            <ControlsPanel>
                <ControlRow label="Area label">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| area_label.set(event_target_value(&e))
                    >
                        <option value="Right-click me" selected>"Right-click me"</option>
                        <option value="Right-click anywhere here">"Right-click anywhere here"</option>
                        <option value="Context menu area">"Context menu area"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Show separator">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| show_separator.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
