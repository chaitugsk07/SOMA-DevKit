use leptos::prelude::*;

#[component]
pub fn PageShell(
    #[prop(into)] title: Signal<String>,
    #[prop(into)] subtitle: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{subtitle}</p>
            </div>
            {children()}
        </div>
    }
}

#[component]
pub fn PreviewPanel(children: Children) -> impl IntoView {
    view! {
        <div class="bg-card border border-border rounded-md p-6 md:p-12 flex items-center justify-center min-h-52">
            {children()}
        </div>
    }
}

#[component]
pub fn ControlsPanel(children: Children) -> impl IntoView {
    view! {
        <div class="bg-card border border-border rounded-md p-6">
            <h2 class="text-sm font-semibold text-foreground mb-4">"Controls"</h2>
            {children()}
        </div>
    }
}

#[component]
pub fn ControlRow(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3 border-b border-border last:border-0">
            <span class="text-sm text-muted-foreground">{label}</span>
            {children()}
        </div>
    }
}
