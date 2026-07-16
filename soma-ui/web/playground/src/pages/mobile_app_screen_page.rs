use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::MobileAppScreen;

#[component]
pub fn MobileAppScreenPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <div>
            <h1 class="font-heading text-2xl font-bold text-foreground">{move || s().screen_mobile_title}</h1>
            <p class="text-sm text-muted-foreground mt-1 mb-4">{move || s().screen_mobile_subtitle}</p>
            <p class="text-xs text-muted-foreground mb-6">"Use the viewport toolbar at the top of the app to test responsiveness."</p>

            // Phone frame
            <div class="mx-auto max-w-[390px] rounded-[2rem] border-4 border-border overflow-hidden shadow-2xl">
                <div class="h-[760px] overflow-auto">
                    <MobileAppScreen />
                </div>
            </div>
        </div>
    }
}
