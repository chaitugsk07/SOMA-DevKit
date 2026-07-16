use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Hero;

#[component]
pub fn HeroPage() -> impl IntoView {
    view! {
        <PageShell
            title=Signal::derive(|| "Hero".to_string())
            subtitle=Signal::derive(|| "Marketing hero section with eyebrow, title, subtitle, and CTA slots.".to_string())
        >
            <PreviewPanel>
                <Hero
                    eyebrow="Introducing".to_string()
                    title="Build faster with Soma UI".to_string()
                    subtitle="A Leptos component library with a Palantir-inspired design system.".to_string()
                >
                    <button class="bg-primary text-primary-foreground px-5 py-2 rounded-md text-sm font-medium">"Get Started"</button>
                    <button class="border border-border text-foreground px-5 py-2 rounded-md text-sm font-medium hover:bg-accent transition-colors">"Learn More"</button>
                </Hero>
            </PreviewPanel>
        </PageShell>
    }
}
