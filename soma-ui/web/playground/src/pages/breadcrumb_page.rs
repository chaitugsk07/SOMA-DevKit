use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};

#[component]
pub fn BreadcrumbNavPage() -> impl IntoView {
    // depth: 2 = Home > Current, 3 = Home > Mid > Current, 4 = Home > Mid1 > Mid2 > Current
    let depth = RwSignal::new(3usize);

    view! {
        <PageShell
            title=Signal::derive(move || "Breadcrumb".to_string())
            subtitle=Signal::derive(move || "A composable navigation trail showing the current page location.".to_string())
        >
            <PreviewPanel>
                {move || {
                    let d = depth.get();
                    view! {
                        <Breadcrumb>
                            <BreadcrumbItem>
                                <BreadcrumbLink href="/".to_string()>"Home"</BreadcrumbLink>
                            </BreadcrumbItem>
                            {(d >= 3).then(|| view! {
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbLink href="/components".to_string()>"Components"</BreadcrumbLink>
                                </BreadcrumbItem>
                            })}
                            {(d >= 4).then(|| view! {
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbLink href="/components/navigation".to_string()>"Navigation"</BreadcrumbLink>
                                </BreadcrumbItem>
                            })}
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                            </BreadcrumbItem>
                        </Breadcrumb>
                    }
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Depth (2–4 levels)">
                    <div class="flex items-center gap-3">
                        <input
                            type="range"
                            min="2"
                            max="4"
                            value="3"
                            class="w-32 accent-primary"
                            on:input=move |e| {
                                if let Ok(n) = event_target_value(&e).parse::<usize>() {
                                    depth.set(n);
                                }
                            }
                        />
                        <span class="text-sm text-foreground font-mono w-4">{move || depth.get()}</span>
                    </div>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
