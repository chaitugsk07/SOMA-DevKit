use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use leptos::prelude::*;

#[component]
pub fn ConsoleShell(
    #[prop(into)] brand: String,
    #[prop(optional, into)] version: Option<String>,
    sidebar: ChildrenFn,
    topbar: ChildrenFn,
    #[prop(optional)] sidebar_footer: Option<ChildrenFn>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let sidebar_open = RwSignal::new(false);
    let sidebar = StoredValue::new(sidebar);
    let topbar = StoredValue::new(topbar);
    let sidebar_footer = StoredValue::new(sidebar_footer);
    let root_class = format!(
        "flex h-screen min-h-0 bg-background text-foreground overflow-hidden {}",
        class
    );

    let brand_row = move |mobile: bool| {
        let version = version.clone();
        view! {
            <div class="flex items-center gap-2 px-4 h-14 border-b border-border shrink-0">
                <svg aria-hidden="true" width="20" height="20" viewBox="0 0 56 56" fill="none">
                    <polygon
                        points="28,4 52,16 52,40 28,52 4,40 4,16"
                        stroke="hsl(var(--primary))"
                        stroke-width="2.5"
                        fill="hsl(var(--primary) / 0.1)"
                    />
                </svg>
                <span class="font-heading font-bold text-sm text-foreground">{brand.clone()}</span>
                {version.map(|value| view! {
                    <span class="ms-auto font-mono text-xs bg-primary/10 text-primary px-1.5 py-0.5 rounded">
                        {value}
                    </span>
                })}
                {mobile.then(|| view! {
                    <button
                        type="button"
                        class=format!("ms-auto inline-flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:bg-accent hover:text-foreground {} {}", CONTROL_MOTION, FOCUS_RING)
                        aria-label="Close navigation"
                        on:click=move |_| sidebar_open.set(false)
                    >
                        <span aria-hidden="true">"×"</span>
                    </button>
                })}
            </div>
        }
    };

    view! {
        <div class=root_class>
            <aside class="hidden md:flex w-60 shrink-0 flex-col border-e border-border bg-card">
                {brand_row(false)}
                <nav class="flex-1 overflow-y-auto" aria-label="Primary navigation">
                    {sidebar.with_value(|content| content())}
                </nav>
                {sidebar_footer.with_value(|footer| footer.as_ref().map(|content| content()))}
            </aside>

            <Show when=move || sidebar_open.get()>
                <div class="fixed inset-0 z-50 flex md:hidden">
                    <button
                        type="button"
                        class="absolute inset-0 bg-black/60"
                        aria-label="Close navigation"
                        on:click=move |_| sidebar_open.set(false)
                    />
                    <aside class="relative z-10 flex h-full w-60 flex-col border-e border-border bg-card shadow-elev-lg">
                        {brand_row(true)}
                        <nav
                            class="flex-1 overflow-y-auto"
                            aria-label="Primary navigation"
                            on:click=move |_| sidebar_open.set(false)
                        >
                            {sidebar.with_value(|content| content())}
                        </nav>
                        {sidebar_footer.with_value(|footer| footer.as_ref().map(|content| content()))}
                    </aside>
                </div>
            </Show>

            <div class="flex min-w-0 flex-1 flex-col overflow-hidden">
                <header class="flex h-14 shrink-0 items-center gap-3 border-b border-border bg-background px-3 md:px-4">
                    <button
                        type="button"
                        class=format!("inline-flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:bg-accent hover:text-foreground md:hidden {} {}", CONTROL_MOTION, FOCUS_RING)
                        aria-label="Open navigation"
                        aria-expanded=move || sidebar_open.get().to_string()
                        on:click=move |_| sidebar_open.set(true)
                    >
                        <svg aria-hidden="true" width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M2 3.25h12v1.5H2zm0 4h12v1.5H2zm0 4h12v1.5H2z" />
                        </svg>
                    </button>
                    <div class="flex min-w-0 flex-1 items-center gap-3">
                        {topbar.with_value(|content| content())}
                    </div>
                </header>

                <main class="min-h-0 flex-1 overflow-y-auto">
                    <div class="mx-auto w-full max-w-[1400px]">
                        {children()}
                    </div>
                </main>
            </div>
        </div>
    }
}
