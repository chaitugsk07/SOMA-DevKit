use crate::ui::*;
use leptos::prelude::*;
use soma_ui::InputOtp;

fn parse_length(s: &str) -> usize {
    match s {
        "4" => 4,
        "8" => 8,
        _ => 6,
    }
}

#[component]
pub fn InputOtpPage() -> impl IntoView {
    let code = RwSignal::new(String::new());
    let length = RwSignal::new(6usize);

    view! {
        <PageShell
            title=Signal::derive(move || "Input OTP".to_string())
            subtitle=Signal::derive(move || "Numeric one-time password input. Focus advances automatically on entry, retreats on Backspace.".to_string())
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-8 min-h-72">
                <div class="flex flex-col items-center gap-3">
                    {move || view! {
                        <InputOtp value=code length=length.get() />
                    }}
                    <p class="text-sm text-muted-foreground">
                        "Value: "
                        <span class="text-foreground font-mono tracking-widest">
                            {move || {
                                let v = code.get();
                                if v.is_empty() { "——".to_string() } else { v }
                            }}
                        </span>
                    </p>
                </div>
            </div>

            // Controls
            <ControlsPanel>
                <ControlRow label="Length">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            length.set(parse_length(&event_target_value(&e)));
                            code.set(String::new());
                        }
                    >
                        <option value="4">"4 digits (PIN)"</option>
                        <option value="6" selected>"6 digits (OTP)"</option>
                        <option value="8">"8 digits"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Clear value">
                    <button
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm hover:bg-accent"
                        on:click=move |_| code.set(String::new())
                    >
                        "Clear"
                    </button>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
