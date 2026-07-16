use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink,
    NavigationMenuTrigger,
};

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    let show_company = RwSignal::new(true);
    // link_count controls how many Product links are shown (1–5)
    let link_count = RwSignal::new(3usize);

    view! {
        <PageShell
            title=Signal::derive(move || "Navigation Menu".to_string())
            subtitle=Signal::derive(move || "Site-level horizontal nav with dropdown panels. Click a trigger or hover when one is open.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 min-h-72 flex items-start justify-center">
                <NavigationMenu>
                    <NavigationMenuItem value="products".to_string()>
                        <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <div class="space-y-1">
                                <p class="text-xs font-semibold uppercase tracking-wider text-muted-foreground px-3 py-1">"Core"</p>
                                <NavigationMenuLink href="#">"Dashboard"</NavigationMenuLink>
                                <Show when=move || { link_count.get() >= 2 }>
                                    <NavigationMenuLink href="#">"Analytics"</NavigationMenuLink>
                                </Show>
                                <Show when=move || { link_count.get() >= 3 }>
                                    <NavigationMenuLink href="#">"Reporting"</NavigationMenuLink>
                                </Show>
                                <Show when=move || { link_count.get() >= 4 }>
                                    <NavigationMenuLink href="#">"Exports"</NavigationMenuLink>
                                </Show>
                                <Show when=move || { link_count.get() >= 5 }>
                                    <NavigationMenuLink href="#">"Webhooks"</NavigationMenuLink>
                                </Show>
                            </div>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem value="docs".to_string()>
                        <NavigationMenuTrigger>"Docs"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <div class="space-y-1">
                                <NavigationMenuLink href="#">"Getting Started"</NavigationMenuLink>
                                <NavigationMenuLink href="#">"API Reference"</NavigationMenuLink>
                                <NavigationMenuLink href="#">"Examples"</NavigationMenuLink>
                                <NavigationMenuLink href="#">"Changelog"</NavigationMenuLink>
                            </div>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <Show when=move || show_company.get()>
                        <NavigationMenuItem value="company".to_string()>
                            <NavigationMenuTrigger>"Company"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <div class="space-y-1">
                                    <p class="text-xs font-semibold uppercase tracking-wider text-muted-foreground px-3 py-1">"About"</p>
                                    <NavigationMenuLink href="#">"Our Story"</NavigationMenuLink>
                                    <NavigationMenuLink href="#">"Team"</NavigationMenuLink>
                                    <NavigationMenuLink href="#">"Careers"</NavigationMenuLink>
                                    <NavigationMenuLink href="#">"Blog"</NavigationMenuLink>
                                </div>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                    </Show>
                </NavigationMenu>
            </div>

            <ControlsPanel>
                <ControlRow label="Products links (1–5)">
                    <div class="flex items-center gap-3">
                        <input
                            type="range"
                            min="1"
                            max="5"
                            value="3"
                            class="w-32 accent-primary"
                            on:input=move |e| {
                                if let Ok(n) = event_target_value(&e).parse::<usize>() {
                                    link_count.set(n);
                                }
                            }
                        />
                        <span class="text-sm text-foreground font-mono w-4">{move || link_count.get()}</span>
                    </div>
                </ControlRow>
                <ControlRow label="Show Company menu">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| show_company.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
