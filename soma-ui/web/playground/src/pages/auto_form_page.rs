use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{AutoForm, AutoFormField, AutoFormFieldKind};
use std::collections::HashMap;

#[component]
pub fn AutoFormPage() -> impl IntoView {
    let result = RwSignal::new(Option::<HashMap<String, String>>::None);

    let fields = vec![
        AutoFormField {
            name: "name".to_string(),
            label: "Full Name".to_string(),
            kind: AutoFormFieldKind::Text,
            required: true,
            placeholder: "Jane Doe".to_string(),
        },
        AutoFormField {
            name: "email".to_string(),
            label: "Email".to_string(),
            kind: AutoFormFieldKind::Email,
            required: true,
            placeholder: "you@example.com".to_string(),
        },
        AutoFormField {
            name: "age".to_string(),
            label: "Age".to_string(),
            kind: AutoFormFieldKind::Number,
            required: false,
            placeholder: "30".to_string(),
        },
        AutoFormField {
            name: "role".to_string(),
            label: "Role".to_string(),
            kind: AutoFormFieldKind::Select(vec![
                ("admin".to_string(), "Admin".to_string()),
                ("editor".to_string(), "Editor".to_string()),
                ("viewer".to_string(), "Viewer".to_string()),
            ]),
            required: true,
            placeholder: "Pick a role…".to_string(),
        },
        AutoFormField {
            name: "newsletter".to_string(),
            label: "Subscribe to newsletter".to_string(),
            kind: AutoFormFieldKind::Checkbox,
            required: false,
            placeholder: String::new(),
        },
    ];

    let on_submit = Callback::new(move |map: HashMap<String, String>| {
        result.set(Some(map));
    });

    view! {
        <PageShell
            title=Signal::derive(move || "Auto Form".to_string())
            subtitle=Signal::derive(move || "Schema-driven form built from a Vec<AutoFormField> spec. Upgrade path: proc-macro derive.".to_string())
        >
            // Preview (bespoke: no flex centering)
            <div class="bg-card border border-border rounded-md p-6 md:p-12">
                <div class="w-full max-w-sm mx-auto">
                    <AutoForm fields=fields on_submit=on_submit submit_label="Create Account".to_string() />
                </div>
            </div>

            <Show when=move || result.get().is_some()>
                <div class="rounded-md border border-border bg-muted p-4 text-sm text-foreground space-y-1">
                    <p class="font-medium">"Submitted values:"</p>
                    {move || result.get().map(|map| {
                        let mut entries: Vec<_> = map.into_iter().collect();
                        entries.sort_by(|a, b| a.0.cmp(&b.0));
                        view! {
                            <dl class="space-y-0.5">
                                <For
                                    each=move || entries.clone()
                                    key=|(k, _)| k.clone()
                                    children=|(k, v)| view! {
                                        <div class="flex gap-2">
                                            <dt class="text-muted-foreground min-w-24">{k}</dt>
                                            <dd class="text-foreground">{v}</dd>
                                        </div>
                                    }
                                />
                            </dl>
                        }
                    })}
                </div>
            </Show>
        </PageShell>
    }
}
