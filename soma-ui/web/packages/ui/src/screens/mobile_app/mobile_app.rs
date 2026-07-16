use crate::{BottomNav, BottomNavItem};
use leptos::prelude::*;

#[component]
pub fn MobileAppScreen() -> impl IntoView {
    let active_tab = RwSignal::new(0usize);

    view! {
        <div class="flex flex-col h-full bg-background relative">
            // Top bar
            <header class="flex items-center justify-between px-4 py-3 bg-card border-b border-border shrink-0">
                <h1 class="font-heading font-semibold text-sm text-foreground">"FOUNDRY MOBILE"</h1>
                <div class="flex items-center gap-2">
                    <button class="w-8 h-8 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-accent"
                        inner_html=r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001q.044.06.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1 1 0 0 0-.115-.099zm-5.242 1.656a5.5 5.5 0 1 1 0-11 5.5 5.5 0 0 1 0 11"/></svg>"# />
                    <button class="w-8 h-8 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-accent"
                        inner_html=r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 16a2 2 0 0 0 2-2H6a2 2 0 0 0 2 2M8 1.918l-.797.161A4 4 0 0 0 4 6c0 .628-.134 2.197-.459 3.742-.16.767-.376 1.566-.663 2.258h10.244c-.287-.692-.502-1.49-.663-2.258C12.134 8.197 12 6.628 12 6a4 4 0 0 0-3.203-3.92zM14.22 12c.223.447.481.801.78 1H1c.299-.199.557-.553.78-1C2.68 10.2 3 6.88 3 6a5 5 0 0 1 10 0c0 .88.32 4.2 1.22 6"/></svg>"# />
                </div>
            </header>

            // Scrollable content
            <div class="flex-1 overflow-y-auto p-4 flex flex-col gap-3">
                // Status bar
                <div class="flex items-center justify-between p-3 rounded-md bg-card border border-border">
                    <div class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-green-500"></span>
                        <span class="font-mono text-xs text-foreground">"OPERATIONAL"</span>
                    </div>
                    <span class="font-mono text-xs text-muted-foreground">"12 active"</span>
                </div>

                // Stat cards
                <div class="grid grid-cols-2 gap-3">
                    <div class="rounded-md bg-card border border-border p-3">
                        <p class="text-xs text-muted-foreground uppercase tracking-widest font-heading mb-1">"PIPELINES"</p>
                        <p class="text-xl font-mono tabular-nums font-bold text-foreground">"1,284"</p>
                        <p class="text-xs text-green-500 font-mono mt-0.5">"+12%"</p>
                    </div>
                    <div class="rounded-md bg-card border border-border p-3">
                        <p class="text-xs text-muted-foreground uppercase tracking-widest font-heading mb-1">"LATENCY"</p>
                        <p class="text-xl font-mono tabular-nums font-bold text-foreground">"142ms"</p>
                        <p class="text-xs text-green-500 font-mono mt-0.5">"-5.1%"</p>
                    </div>
                </div>

                // Pipeline list
                <div class="flex flex-col gap-0 rounded-md bg-card border border-border overflow-hidden">
                    {vec![
                        ("pipeline-alpha-7", "Running", true),
                        ("etl-transform-14", "Running", true),
                        ("report-daily-fin", "Queued", false),
                        ("sync-crm-contacts", "Failed", false),
                    ].into_iter().map(|(name, status, ok)| {
                        view! {
                            <div class="flex items-center justify-between px-3 py-2.5 border-b border-border last:border-b-0">
                                <span class="font-mono text-xs text-foreground">{name}</span>
                                <div class="flex items-center gap-1.5">
                                    <span class=move || if ok { "w-1.5 h-1.5 rounded-full bg-green-500" } else { "w-1.5 h-1.5 rounded-full bg-red-500" }></span>
                                    <span class="text-xs text-muted-foreground">{status}</span>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // Bottom nav (fixed to bottom of the phone container)
            <BottomNav>
                <BottomNavItem label="Home".to_string() active=Signal::derive(move || active_tab.get() == 0) on_click=Callback::new(move |_| active_tab.set(0))>
                    <span inner_html=r#"<svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor"><path d="M8.354 1.146a.5.5 0 0 0-.708 0l-6 6A.5.5 0 0 0 1.5 7.5v7a.5.5 0 0 0 .5.5h4.5v-5h3v5H14a.5.5 0 0 0 .5-.5v-7a.5.5 0 0 0-.146-.354z"/></svg>"# />
                </BottomNavItem>
                <BottomNavItem label="Pipelines".to_string() active=Signal::derive(move || active_tab.get() == 1) on_click=Callback::new(move |_| active_tab.set(1))>
                    <span inner_html=r#"<svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor"><path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/></svg>"# />
                </BottomNavItem>
                <BottomNavItem label="Alerts".to_string() active=Signal::derive(move || active_tab.get() == 2) on_click=Callback::new(move |_| active_tab.set(2))>
                    <span inner_html=r#"<svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor"><path d="M8 16a2 2 0 0 0 2-2H6a2 2 0 0 0 2 2M8 1.918l-.797.161A4 4 0 0 0 4 6c0 .628-.134 2.197-.459 3.742-.16.767-.376 1.566-.663 2.258h10.244c-.287-.692-.502-1.49-.663-2.258C12.134 8.197 12 6.628 12 6a4 4 0 0 0-3.203-3.92z"/></svg>"# />
                </BottomNavItem>
                <BottomNavItem label="Profile".to_string() active=Signal::derive(move || active_tab.get() == 3) on_click=Callback::new(move |_| active_tab.set(3))>
                    <span inner_html=r#"<svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor"><path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6m2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0m4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4m-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.029 10 8 10s-3.516.68-4.168 1.332c-.678.678-.83 1.418-.832 1.664z"/></svg>"# />
                </BottomNavItem>
            </BottomNav>
        </div>
    }
}
