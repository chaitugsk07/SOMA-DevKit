use leptos::prelude::*;

#[component]
pub fn AppShell(
    sidebar: AnyView,
    #[prop(optional)] header_end: Option<AnyView>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("flex h-screen overflow-hidden bg-background {}", class);
    view! {
        <div class=combined>
            {sidebar}
            <div class="flex min-w-0 flex-1 flex-col overflow-hidden">
                <header class="flex h-[58px] items-center justify-end gap-3 border-b border-border bg-background/80 px-4 backdrop-blur-md">
                    {header_end}
                </header>
                <main class="flex-1 overflow-y-auto">
                    {children()}
                </main>
            </div>
        </div>
    }
}
