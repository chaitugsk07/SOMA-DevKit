use crate::i18n::{strings, Locale};
use leptos::prelude::*;

#[component]
pub fn ThemePage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <div class="max-w-3xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().theme_title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().theme_subtitle}</p>
            </div>

            // Colors
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().theme_colors}</h2>
                <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
                    // background
                    <div>
                        <div class="h-16 rounded-md border border-border bg-background flex items-center justify-center">
                            <span class="text-xs text-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"background"</p>
                    </div>
                    // card
                    <div>
                        <div class="h-16 rounded-md border border-border bg-card flex items-center justify-center">
                            <span class="text-xs text-card-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"card"</p>
                    </div>
                    // muted
                    <div>
                        <div class="h-16 rounded-md border border-border bg-muted flex items-center justify-center">
                            <span class="text-xs text-muted-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"muted"</p>
                    </div>
                    // secondary
                    <div>
                        <div class="h-16 rounded-md border border-border bg-secondary flex items-center justify-center">
                            <span class="text-xs text-secondary-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"secondary"</p>
                    </div>
                    // accent
                    <div>
                        <div class="h-16 rounded-md border border-border bg-accent flex items-center justify-center">
                            <span class="text-xs text-accent-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"accent"</p>
                    </div>
                    // primary
                    <div>
                        <div class="h-16 rounded-md border border-border bg-primary flex items-center justify-center">
                            <span class="text-xs text-primary-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"primary"</p>
                    </div>
                    // destructive
                    <div>
                        <div class="h-16 rounded-md border border-border bg-destructive flex items-center justify-center">
                            <span class="text-xs text-destructive-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"destructive"</p>
                    </div>
                    // success
                    <div>
                        <div class="h-16 rounded-md border border-border bg-success flex items-center justify-center">
                            <span class="text-xs text-success-foreground font-mono">"Aa"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"success"</p>
                    </div>
                    // border (just a swatch to show the border color)
                    <div>
                        <div class="h-16 rounded-md border-4 border-border bg-background flex items-center justify-center">
                            <span class="text-xs text-muted-foreground font-mono">"—"</span>
                        </div>
                        <p class="text-xs text-muted-foreground mt-1">"border"</p>
                    </div>
                </div>
            </div>

            // Typography
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().theme_typography}</h2>
                <div class="space-y-6">
                    // Heading font
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3">"font-heading · Rajdhani"</p>
                        <div class="space-y-1">
                            <p class="font-heading font-bold text-2xl text-foreground">"The quick brown fox"</p>
                            <p class="font-heading font-semibold text-xl text-foreground">"The quick brown fox"</p>
                            <p class="font-heading font-medium text-lg text-foreground">"The quick brown fox"</p>
                            <p class="font-heading text-base text-foreground">"The quick brown fox"</p>
                        </div>
                    </div>
                    // Body font
                    <div class="border border-border rounded-md p-4">
                        <p class="text-xs text-muted-foreground mb-3">"font-sans · Outfit"</p>
                        <div class="space-y-1">
                            <p class="font-sans font-semibold text-lg text-foreground">"The quick brown fox jumps over the lazy dog"</p>
                            <p class="font-sans text-base text-foreground">"The quick brown fox jumps over the lazy dog"</p>
                            <p class="font-sans text-sm text-muted-foreground">"The quick brown fox jumps over the lazy dog"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
