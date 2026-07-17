use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::DashboardScreen;

#[component]
pub fn DashboardScreenPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <div class="max-w-5xl">
            <h1 class="font-heading text-2xl font-bold text-foreground">{move || s().screen_dashboard_title}</h1>
            <p class="text-sm text-muted-foreground mt-1 mb-4">{move || s().screen_dashboard_subtitle}</p>
            <p class="text-xs text-muted-foreground mb-4">"Use the viewport toolbar at the top of the app to test responsiveness."</p>

            <div class="rounded-md border border-border overflow-hidden">
                <div class="h-9 bg-muted flex items-center gap-1.5 px-3 border-b border-border">
                    <span class="w-3 h-3 rounded-full bg-border"></span>
                    <span class="w-3 h-3 rounded-full bg-border"></span>
                    <span class="w-3 h-3 rounded-full bg-border"></span>
                    <div class="ms-3 flex-1 max-w-xs h-5 rounded bg-background/60 border border-border flex items-center px-2">
                        <span class="text-xs text-muted-foreground font-mono">"console.soma.dev/overview"</span>
                    </div>
                </div>
                <div class="h-[640px] overflow-auto">
                    <DashboardScreen />
                </div>
            </div>
        </div>
    }
}
