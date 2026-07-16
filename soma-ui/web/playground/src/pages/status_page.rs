use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Status, StatusKind};

fn parse_kind(s: &str) -> StatusKind {
    match s {
        "Offline" => StatusKind::Offline,
        "Away" => StatusKind::Away,
        "Busy" => StatusKind::Busy,
        _ => StatusKind::Online,
    }
}

#[component]
pub fn StatusPage() -> impl IntoView {
    let kind = RwSignal::new(StatusKind::Online);
    let label_text = RwSignal::new("Online".to_string());
    let show_label = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(|| "Status".to_string())
            subtitle=Signal::derive(|| "Colored dot indicator with optional label.".to_string())
        >
            // Preview
            <PreviewPanel>
                {move || {
                    if show_label.get() {
                        view! { <Status kind=kind.get() label=label_text.get() /> }.into_any()
                    } else {
                        view! { <Status kind=kind.get() /> }.into_any()
                    }
                }}
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Kind">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            let val = event_target_value(&e);
                            kind.set(parse_kind(&val));
                            label_text.set(val);
                        }
                    >
                        <option value="Online">"Online"</option>
                        <option value="Offline">"Offline"</option>
                        <option value="Away">"Away"</option>
                        <option value="Busy">"Busy"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Label text">
                    <input
                        type="text"
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring w-36"
                        prop:value=move || label_text.get()
                        on:input=move |e| label_text.set(event_target_value(&e))
                    />
                </ControlRow>
                <ControlRow label="Show label">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_label.get()
                        on:change=move |e| show_label.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>

            // All Variants
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="flex flex-wrap gap-6">
                    <Status kind=StatusKind::Online label="Online".to_string() />
                    <Status kind=StatusKind::Offline label="Offline".to_string() />
                    <Status kind=StatusKind::Away label="Away".to_string() />
                    <Status kind=StatusKind::Busy label="Busy".to_string() />
                </div>
            </div>
        </PageShell>
    }
}
