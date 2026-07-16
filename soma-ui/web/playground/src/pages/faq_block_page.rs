use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::FaqBlock;

#[component]
pub fn FaqBlockPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().block_faq_title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().block_faq_subtitle}</p>
            </div>
            <div class="bg-card border border-border rounded-md overflow-hidden">
                <FaqBlock />
            </div>
        </div>
    }
}
