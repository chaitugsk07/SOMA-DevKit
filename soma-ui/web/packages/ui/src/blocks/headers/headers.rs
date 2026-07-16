use crate::components::data_display::avatar::Avatar;
use crate::components::inputs::button::{Button, ButtonSize, ButtonVariant};
use crate::components::inputs::input::Input;
use leptos::prelude::*;

#[component]
pub fn HeaderBlock() -> impl IntoView {
    let search = RwSignal::new(String::new());

    view! {
        <header class="bg-card border-b border-border">
            <div class="flex items-center gap-4 px-6 h-16">
                // Brand
                <span class="font-heading font-bold text-foreground shrink-0">"Soma UI"</span>

                // Nav links (hidden on small)
                <nav class="hidden md:flex items-center gap-1 ms-4">
                    <a href="#" class="px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-accent rounded-md transition-colors">"Products"</a>
                    <a href="#" class="px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-accent rounded-md transition-colors">"Docs"</a>
                    <a href="#" class="px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-accent rounded-md transition-colors">"Blog"</a>
                    <a href="#" class="px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-accent rounded-md transition-colors">"Pricing"</a>
                </nav>

                // Spacer
                <div class="flex-1" />

                // Actions
                <div class="flex items-center gap-2">
                    <div class="hidden sm:block w-48">
                        <Input placeholder="Search…".to_string() value=search />
                    </div>
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon aria_label="Notifications".to_string()>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
                            <path d="M8 16a2 2 0 0 0 2-2H6a2 2 0 0 0 2 2M8 1.918l-.797.161A4 4 0 0 0 4 6c0 .628-.134 2.197-.459 3.742-.16.767-.376 1.566-.663 2.258h10.244c-.287-.692-.502-1.49-.663-2.258C12.134 8.197 12 6.628 12 6a4 4 0 0 0-3.203-3.92zM14.22 12c.223.447.481.801.78 1H1c.299-.199.557-.553.78-1C2.68 10.2 3 6.88 3 6a5 5 0 0 1 10 0c0 2.88.32 4.2 1.22 6"/>
                        </svg>
                    </Button>
                    <Avatar fallback="JD".to_string() />
                </div>
            </div>
        </header>
    }
}
