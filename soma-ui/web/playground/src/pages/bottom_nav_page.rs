use crate::ui::*;
use leptos::prelude::*;
use soma_ui::icons::{icondata, Icon};
use soma_ui::{BottomNav, BottomNavItem};

#[component]
pub fn BottomNavPage() -> impl IntoView {
    let active = RwSignal::new("home");

    view! {
        <PageShell
            title=Signal::derive(|| "Bottom Nav".to_string())
            subtitle=Signal::derive(|| "A mobile bottom navigation bar with icon + label items.".to_string())
        >
            // Phone frame preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex items-center justify-center min-h-52">
                <div class="w-64 rounded-2xl border border-border overflow-hidden shadow-lg bg-background">
                    <div class="h-48 flex items-center justify-center text-sm text-muted-foreground bg-muted/30">
                        "Content area"
                    </div>
                    <BottomNav>
                        <BottomNavItem
                            label="Home".to_string()
                            active=Signal::derive(move || active.get() == "home")
                            on_click=Callback::new(move |_| active.set("home"))
                        >
                            <Icon icon=Signal::derive(|| icondata::LuHouse) width="20" height="20" />
                        </BottomNavItem>
                        <BottomNavItem
                            label="Search".to_string()
                            active=Signal::derive(move || active.get() == "search")
                            on_click=Callback::new(move |_| active.set("search"))
                        >
                            <Icon icon=Signal::derive(|| icondata::LuSearch) width="20" height="20" />
                        </BottomNavItem>
                        <BottomNavItem
                            label="Inbox".to_string()
                            active=Signal::derive(move || active.get() == "inbox")
                            on_click=Callback::new(move |_| active.set("inbox"))
                        >
                            <Icon icon=Signal::derive(|| icondata::LuInbox) width="20" height="20" />
                        </BottomNavItem>
                        <BottomNavItem
                            label="Profile".to_string()
                            active=Signal::derive(move || active.get() == "profile")
                            on_click=Callback::new(move |_| active.set("profile"))
                        >
                            <Icon icon=Signal::derive(|| icondata::LuUser) width="20" height="20" />
                        </BottomNavItem>
                    </BottomNav>
                </div>
            </div>

            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
                <div class="flex items-center justify-between py-3">
                    <span class="text-sm text-muted-foreground">"Active item"</span>
                    <span class="text-sm text-foreground font-mono">{move || active.get()}</span>
                </div>
            </div>
        </PageShell>
    }
}
