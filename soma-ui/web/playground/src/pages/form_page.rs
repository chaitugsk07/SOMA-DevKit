use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Button, Field, Form, Input};

#[component]
pub fn FormPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let submitted = RwSignal::new(Option::<(String, String)>::None);

    let on_submit = Callback::new(move |()| {
        submitted.set(Some((email.get(), password.get())));
    });

    view! {
        <PageShell
            title=Signal::derive(move || "Form".to_string())
            subtitle=Signal::derive(move || "Form wrapper with prevent_default and optional submit callback.".to_string())
        >
            <PreviewPanel>
                <div class="w-full max-w-sm space-y-6">
                    <Form on_submit=on_submit>
                        <Field label="Email" for_id="form-email".to_string()>
                            <Input value=email input_type="email".to_string() placeholder="you@example.com".to_string() />
                        </Field>
                        <Field label="Password" for_id="form-password".to_string()>
                            <Input value=password input_type="password".to_string() placeholder="••••••••".to_string() />
                        </Field>
                        <Button>"Sign in"</Button>
                    </Form>

                    <Show when=move || submitted.get().is_some()>
                        <div class="rounded-md border border-border bg-muted p-4 text-sm text-foreground space-y-1">
                            <p class="font-medium">"Submitted:"</p>
                            {move || submitted.get().map(|(e, _)| view! {
                                <p class="text-muted-foreground">"Email: " {e}</p>
                            })}
                        </div>
                    </Show>
                </div>
            </PreviewPanel>
        </PageShell>
    }
}
