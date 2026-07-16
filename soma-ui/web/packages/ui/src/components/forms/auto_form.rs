use crate::{Button, Checkbox, Field, Form, Input, Select, SelectContent, SelectItem, Textarea};
use leptos::prelude::*;
use std::collections::HashMap;

// ponytail: Runtime schema-driven form. Takes a Vec<AutoFormField> spec and builds
// the form UI, collecting values into a HashMap<String,String>.
// Ceiling: derive-macro version (proc-macro crate) is the documented upgrade path
// for zero-boilerplate struct-to-form generation.
// Validation ceiling: only checks required fields are non-empty; no regex/type checks.

#[derive(Clone)]
pub enum AutoFormFieldKind {
    Text,
    Email,
    Password,
    Number,
    Textarea,
    Checkbox,
    Select(Vec<(String, String)>), // (value, label) pairs
}

#[derive(Clone)]
pub struct AutoFormField {
    pub name: String,
    pub label: String,
    pub kind: AutoFormFieldKind,
    pub required: bool,
    pub placeholder: String,
}

#[component]
pub fn AutoForm(
    fields: Vec<AutoFormField>,
    #[prop(optional)] on_submit: Option<Callback<HashMap<String, String>>>,
    #[prop(default = String::from("Submit"))] submit_label: String,
) -> impl IntoView {
    // Internal state: one RwSignal<String> per field (checkbox uses "true"/"false")
    // and one RwSignal<Option<String>> per field for error display
    let field_signals: Vec<(RwSignal<String>, RwSignal<Option<String>>)> = fields
        .iter()
        .map(|f| {
            let default = match &f.kind {
                AutoFormFieldKind::Checkbox => "false".to_string(),
                _ => String::new(),
            };
            (RwSignal::new(default), RwSignal::new(None::<String>))
        })
        .collect();

    let fields = StoredValue::new(fields);
    let field_signals = StoredValue::new(field_signals);
    let submit_label = StoredValue::new(submit_label);

    let handle_submit = move || {
        let specs = fields.get_value();
        let sigs = field_signals.get_value();
        let mut valid = true;
        // Reset errors
        for (_, err_sig) in &sigs {
            err_sig.set(None);
        }
        // Validate required fields
        for (i, spec) in specs.iter().enumerate() {
            if spec.required {
                let val = sigs[i].0.get();
                let empty = match &spec.kind {
                    AutoFormFieldKind::Checkbox => val == "false",
                    _ => val.trim().is_empty(),
                };
                if empty {
                    sigs[i].1.set(Some(format!("{} is required", spec.label)));
                    valid = false;
                }
            }
        }
        if valid {
            let mut map = HashMap::new();
            for (i, spec) in specs.iter().enumerate() {
                map.insert(spec.name.clone(), sigs[i].0.get());
            }
            if let Some(cb) = on_submit {
                cb.run(map);
            }
        }
    };

    view! {
        <Form on_submit=Callback::new(move |()| handle_submit())>
            <For
                each=move || {
                    fields.get_value().into_iter().enumerate().collect::<Vec<_>>()
                }
                key=|(i, _)| *i
                children=move |(i, spec)| {
                    let sigs = field_signals.get_value();
                    let val_sig = sigs[i].0;
                    let err_sig = sigs[i].1;
                    let field_id = format!("auto-field-{}", spec.name);
                    let err_signal = Signal::derive(move || err_sig.get());

                    view! {
                        <Field
                            label=spec.label.clone()
                            for_id=field_id.clone()
                            error=err_signal
                        >
                            {match spec.kind.clone() {
                                AutoFormFieldKind::Textarea => view! {
                                    <Textarea value=val_sig placeholder=spec.placeholder.clone() />
                                }.into_any(),
                                AutoFormFieldKind::Checkbox => {
                                    let checked = RwSignal::new(false);
                                    // Sync checked → val_sig
                                    Effect::new(move |_| {
                                        val_sig.set(if checked.get() { "true".to_string() } else { "false".to_string() });
                                    });
                                    view! {
                                        <Checkbox checked=checked />
                                    }.into_any()
                                },
                                AutoFormFieldKind::Select(opts) => {
                                    let opts = StoredValue::new(opts);
                                    view! {
                                        <Select value=val_sig placeholder=spec.placeholder.clone()>
                                            <SelectContent>
                                                <For
                                                    each=move || opts.get_value()
                                                    key=|(v, _)| v.clone()
                                                    children=|(v, label)| view! {
                                                        <SelectItem value=v>{label}</SelectItem>
                                                    }
                                                />
                                            </SelectContent>
                                        </Select>
                                    }.into_any()
                                },
                                kind => {
                                    let input_type = match kind {
                                        AutoFormFieldKind::Email => "email",
                                        AutoFormFieldKind::Password => "password",
                                        AutoFormFieldKind::Number => "number",
                                        _ => "text",
                                    }.to_string();
                                    view! {
                                        <Input
                                            value=val_sig
                                            input_type=input_type
                                            placeholder=spec.placeholder.clone()
                                        />
                                    }.into_any()
                                }
                            }}
                        </Field>
                    }
                }
            />
            <Button>{move || submit_label.get_value()}</Button>
        </Form>
    }
}
