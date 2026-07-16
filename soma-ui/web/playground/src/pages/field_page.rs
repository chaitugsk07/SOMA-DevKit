use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Field, Input, Select, SelectContent, SelectItem, Textarea};

#[component]
pub fn FieldPage() -> impl IntoView {
    let name_val = RwSignal::new(String::new());
    let bio_val = RwSignal::new(String::new());
    let role_val = RwSignal::new(String::new());
    let email_val = RwSignal::new(String::new());
    let force_error = RwSignal::new(false);

    // Reactive email error: shows when forced or when non-empty and lacks '@'
    let email_error = Signal::derive(move || {
        if force_error.get() {
            return Some("Please enter a valid email address".to_string());
        }
        let v = email_val.get();
        if v.is_empty() || v.contains('@') {
            None
        } else {
            Some("Please enter a valid email address".to_string())
        }
    });

    view! {
        <PageShell
            title=Signal::derive(move || "Field".to_string())
            subtitle=Signal::derive(move || "Accessible field row: label + control + description + reactive error.".to_string())
        >
            // Preview (bespoke: no flex centering)
            <div class="bg-card border border-border rounded-md p-6 md:p-12">
                <div class="space-y-6 max-w-sm mx-auto">
                    <Field label="Full Name" for_id="field-name".to_string() description="Your display name.">
                        <Input value=name_val placeholder="Jane Doe".to_string() />
                    </Field>

                    <Field label="Email" for_id="field-email".to_string() error=email_error>
                        <Input value=email_val input_type="email".to_string() placeholder="you@example.com".to_string() />
                    </Field>

                    <Field label="Bio" for_id="field-bio".to_string() description="Up to 200 characters.">
                        <Textarea value=bio_val placeholder="Tell us about yourself…".to_string() />
                    </Field>

                    <Field label="Role" for_id="field-role".to_string()>
                        <Select value=role_val placeholder="Select a role…".to_string()>
                            <SelectContent>
                                <SelectItem value="admin">"Admin"</SelectItem>
                                <SelectItem value="editor">"Editor"</SelectItem>
                                <SelectItem value="viewer">"Viewer"</SelectItem>
                            </SelectContent>
                        </Select>
                    </Field>
                </div>
            </div>

            <ControlsPanel>
                <ControlRow label="Force email error">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| force_error.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Email input">
                    <span class="text-xs text-muted-foreground font-mono">{move || {
                        let v = email_val.get();
                        if v.is_empty() { "(empty)".to_string() } else { v }
                    }}</span>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
