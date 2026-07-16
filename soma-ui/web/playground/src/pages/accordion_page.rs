use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Accordion, AccordionItem};

#[component]
pub fn AccordionPage() -> impl IntoView {
    let open_by_default = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(|| "Accordion".to_string())
            subtitle=Signal::derive(|| "A vertically stacked set of collapsible sections. Only one section is open at a time.".to_string())
        >
            <PreviewPanel>
                <div class="w-full max-w-md">
                    {move || view! {
                        <Accordion>
                            <AccordionItem value="item-1".to_string() title="What is Soma UI?".to_string() open_by_default=open_by_default.get()>
                                "A Leptos component library built copy-paste-first, so you own the source. Dark-theme only, zinc/blue palette."
                            </AccordionItem>
                            <AccordionItem value="item-2".to_string() title="How do I install it?".to_string()>
                                "Copy the component file into your project. No crate dependency required — that's the copy-paste-first philosophy."
                            </AccordionItem>
                            <AccordionItem value="item-3".to_string() title="Is it accessible?".to_string()>
                                "Yes. Every interactive element has keyboard focus styles and appropriate ARIA attributes."
                            </AccordionItem>
                        </Accordion>
                    }}
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="First item open by default">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || open_by_default.get()
                        on:change=move |e| open_by_default.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <Accordion>
                    <AccordionItem value="a".to_string() title="Default closed item".to_string()>
                        "This item starts closed."
                    </AccordionItem>
                    <AccordionItem value="b".to_string() title="Default open item".to_string() open_by_default=true>
                        "This item starts open because open_by_default=true."
                    </AccordionItem>
                </Accordion>
            </div>
        </PageShell>
    }
}
