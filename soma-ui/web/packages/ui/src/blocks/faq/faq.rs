use crate::components::disclosure::accordion::{Accordion, AccordionItem};
use leptos::prelude::*;

#[component]
pub fn FaqBlock() -> impl IntoView {
    view! {
        <div class="max-w-2xl mx-auto px-6 py-16 space-y-8">
            <div class="text-center space-y-2">
                <h2 class="font-heading text-3xl font-bold tracking-tight text-foreground">"Frequently Asked Questions"</h2>
                <p class="text-muted-foreground">"Everything you need to know about Soma UI."</p>
            </div>

            <Accordion>
                <AccordionItem value="what" title="What is Soma UI?">
                    "Soma UI is a Leptos component library inspired by the Palantir design system. Components are copy-paste friendly — you own the source."
                </AccordionItem>
                <AccordionItem value="how" title="How do I install it?">
                    "Add the crate as a local path dependency in your Cargo.toml. Copy the component files you need directly into your project."
                </AccordionItem>
                <AccordionItem value="framework" title="What frameworks are supported?">
                    "Soma UI targets Leptos 0.7+ with CSR (client-side rendering) via wasm32-unknown-unknown. SSR is not supported out of the box."
                </AccordionItem>
                <AccordionItem value="styles" title="Do I need Tailwind CSS?">
                    "Yes. Components use Tailwind utility classes. Configure Trunk with the Tailwind CSS data-trunk relay for automatic CSS generation."
                </AccordionItem>
                <AccordionItem value="license" title="What is the license?">
                    "MIT. You are free to use, modify, and distribute the components in personal and commercial projects."
                </AccordionItem>
            </Accordion>
        </div>
    }
}
