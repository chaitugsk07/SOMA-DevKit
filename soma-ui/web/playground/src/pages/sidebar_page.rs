use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Sidebar, SidebarItem};

#[component]
pub fn SidebarPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let active = RwSignal::new("/dashboard".to_string());

    let items = vec![
        SidebarItem {
            label: "Dashboard".to_string(),
            href: "/dashboard".to_string(),
            icon: None,
            badge: None,
        },
        SidebarItem {
            label: "Secrets".to_string(),
            href: "/secrets".to_string(),
            icon: None,
            badge: None,
        },
        SidebarItem {
            label: "Config".to_string(),
            href: "/config".to_string(),
            icon: None,
            badge: None,
        },
        SidebarItem {
            label: "Settings".to_string(),
            href: "/settings".to_string(),
            icon: None,
            badge: None,
        },
    ];

    view! {
        <PageShell
            title=Signal::derive(move || s().nav_sidebar.to_string())
            subtitle=Signal::derive(move || "Routed vertical sidebar for app navigation.".to_string())
        >
            <PreviewPanel>
                <div class="h-64 w-64 border border-border rounded-md overflow-hidden">
                    <Sidebar
                        items=items.clone()
                        active_path=Signal::derive(move || active.get())
                    />
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Active path">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || active.get()
                        on:input=move |e| active.set(event_target_value(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
