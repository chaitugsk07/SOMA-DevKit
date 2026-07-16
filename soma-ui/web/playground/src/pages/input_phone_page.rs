use crate::ui::*;
use leptos::prelude::*;
use soma_ui::InputPhone;

#[component]
pub fn InputPhonePage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    // Re-mount the component when reset is triggered to clear internal signals
    let reset_key = RwSignal::new(0u32);

    view! {
        <PageShell
            title=Signal::derive(move || "Input Phone".to_string())
            subtitle=Signal::derive(move || "Country-code selector + phone number field. Strips non-digits from number. Combined value: \"+code number\".".to_string())
        >
            // Preview (bespoke: flex-col min-h-72 gap-4)
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-72">
                <div class="w-full max-w-sm">
                    {move || {
                        let _key = reset_key.get();
                        view! { <InputPhone value=value /> }
                    }}
                </div>
                <p class="text-sm text-muted-foreground">
                    "Value: "
                    <span class="text-foreground font-mono">
                        {move || {
                            let v = value.get();
                            if v.is_empty() { "(empty)".to_string() } else { v }
                        }}
                    </span>
                </p>
            </div>

            <ControlsPanel>
                <ControlRow label="Reset field">
                    <button
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm hover:bg-accent"
                        on:click=move |_| {
                            value.set(String::new());
                            reset_key.update(|k| *k += 1);
                        }
                    >
                        "Reset"
                    </button>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
