use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    Button, DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuTrigger,
};

#[component]
pub fn DropdownMenuPage() -> impl IntoView {
    let trigger_label = RwSignal::new("Open Menu".to_string());
    let show_separator = RwSignal::new(true);
    let last_clicked = RwSignal::new("None".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Dropdown Menu".to_string())
            subtitle=Signal::derive(move || "An anchored menu that opens on click. Closes on outside click or item selection.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-72">
                {move || {
                    let sep = show_separator.get();
                    view! {
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <Button>{trigger_label.get()}</Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuItem on_click=Callback::new(move |_| last_clicked.set("Profile".to_string()))>"Profile"</DropdownMenuItem>
                                <DropdownMenuItem on_click=Callback::new(move |_| last_clicked.set("Settings".to_string()))>"Settings"</DropdownMenuItem>
                                {sep.then(|| view! { <DropdownMenuSeparator /> })}
                                <DropdownMenuItem on_click=Callback::new(move |_| last_clicked.set("Log out".to_string()))>"Log out"</DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    }
                }}
                <p class="text-xs text-muted-foreground">"Last selected: " {move || last_clicked.get()}</p>
            </div>

            <ControlsPanel>
                <ControlRow label="Trigger label">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| trigger_label.set(event_target_value(&e))
                    >
                        <option value="Open Menu" selected>"Open Menu"</option>
                        <option value="Account">"Account"</option>
                        <option value="Options">"Options"</option>
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
