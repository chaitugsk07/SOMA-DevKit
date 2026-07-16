use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Slider;

#[component]
pub fn SliderPage() -> impl IntoView {
    let value = RwSignal::new(50.0f64);
    let min = RwSignal::new(0.0f64);
    let max = RwSignal::new(100.0f64);
    let step = RwSignal::new(1.0f64);
    let disabled = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(move || "Slider".to_string())
            subtitle=Signal::derive(move || "A range input for selecting a numeric value.".to_string())
        >
            <PreviewPanel>
                <div class="w-72 space-y-3">
                    {move || view! {
                        <Slider value=value min=min.get() max=max.get() step=step.get() disabled=disabled.get() />
                    }}
                    <p class="text-sm text-center text-foreground">
                        {move || format!("{:.0}", value.get())}
                    </p>
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Min">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            if let Ok(v) = event_target_value(&e).parse::<f64>() {
                                min.set(v);
                            }
                        }
                    >
                        <option value="0" selected>"0"</option>
                        <option value="-100">"-100"</option>
                        <option value="50">"50"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Max">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            if let Ok(v) = event_target_value(&e).parse::<f64>() {
                                max.set(v);
                            }
                        }
                    >
                        <option value="100" selected>"100"</option>
                        <option value="10">"10"</option>
                        <option value="200">"200"</option>
                        <option value="1000">"1000"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Step">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            if let Ok(v) = event_target_value(&e).parse::<f64>() {
                                step.set(v);
                            }
                        }
                    >
                        <option value="1" selected>"1"</option>
                        <option value="0.1">"0.1"</option>
                        <option value="5">"5"</option>
                        <option value="10">"10"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Disabled">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| disabled.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
