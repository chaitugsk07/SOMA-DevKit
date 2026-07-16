use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

#[component]
pub fn CardPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    let show_description = RwSignal::new(true);
    let show_footer = RwSignal::new(true);
    let title_text = RwSignal::new("Card Title".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || s().card_title.to_string())
            subtitle=Signal::derive(move || s().card_subtitle.to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <Card class="w-full max-w-xs".to_string()>
                        <CardHeader>
                            <CardTitle>{title_text.get()}</CardTitle>
                            {move || show_description.get().then(|| view! {
                                <CardDescription>"Card description goes here."</CardDescription>
                            })}
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm text-foreground">"This is the card content area."</p>
                        </CardContent>
                        {move || show_footer.get().then(|| view! {
                            <CardFooter>
                                <Button>"Action"</Button>
                            </CardFooter>
                        })}
                    </Card>
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Title">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-48"
                        prop:value=move || title_text.get()
                        on:input=move |e| title_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Show description">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_description.get()
                        on:change=move |e| show_description.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Show footer">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_footer.get()
                        on:change=move |e| show_footer.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().all_variants}</h2>
                <div class="grid gap-4">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Simple Card"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm text-foreground">"Just header and content."</p>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Notification"</CardTitle>
                            <CardDescription>"You have a new message."</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm text-foreground">"Your build has completed successfully."</p>
                        </CardContent>
                        <CardFooter class="gap-2".to_string()>
                            <Button>"View"</Button>
                            <Button variant=soma_ui::ButtonVariant::Outline>"Dismiss"</Button>
                        </CardFooter>
                    </Card>
                </div>
            </div>
        </PageShell>
    }
}
