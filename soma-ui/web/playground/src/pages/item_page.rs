use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Item;

#[component]
pub fn ItemPage() -> impl IntoView {
    let name_a = RwSignal::new("Alice Johnson".to_string());
    let email_a = RwSignal::new("alice@example.com".to_string());
    let show_avatar = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Item".to_string())
            subtitle=Signal::derive(move || "Minimal composable row container.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <div class="w-full max-w-sm">
                        <Item>
                            {move || show_avatar.get().then(|| view! {
                                <div class="h-8 w-8 rounded-full bg-muted shrink-0"></div>
                            })}
                            <div class="flex flex-col">
                                <span class="text-sm font-medium text-foreground">{name_a.get()}</span>
                                <span class="text-xs text-muted-foreground">{email_a.get()}</span>
                            </div>
                        </Item>
                    </div>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Name">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || name_a.get()
                        on:input=move |e| name_a.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Email">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || email_a.get()
                        on:input=move |e| email_a.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Show avatar placeholder">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_avatar.get()
                        on:change=move |e| show_avatar.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // Examples
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Examples"</h2>
                <div class="space-y-2">
                    <Item>
                        <div class="h-8 w-8 rounded-full bg-muted shrink-0"></div>
                        <div class="flex flex-col">
                            <span class="text-sm font-medium text-foreground">"Alice Johnson"</span>
                            <span class="text-xs text-muted-foreground">"alice@example.com"</span>
                        </div>
                    </Item>
                    <Item>
                        <div class="h-8 w-8 rounded-full bg-muted shrink-0"></div>
                        <div class="flex flex-col">
                            <span class="text-sm font-medium text-foreground">"Bob Smith"</span>
                            <span class="text-xs text-muted-foreground">"bob@example.com"</span>
                        </div>
                    </Item>
                </div>
            </div>
        </PageShell>
    }
}
