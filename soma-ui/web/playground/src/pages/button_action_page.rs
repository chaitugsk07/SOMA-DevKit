use crate::ui::*;
use leptos::prelude::*;
use soma_ui::ButtonAction;

#[component]
pub fn ButtonActionPage() -> impl IntoView {
    let fired = RwSignal::new(false);
    view! {
        <PageShell
            title=Signal::derive(move || "Button Action".to_string())
            subtitle=Signal::derive(move || "A press-and-hold button that fires an action after a sustained press.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-52">
                <ButtonAction
                    duration_ms=1500
                    on_action=Callback::new(move |_| fired.set(true))
                    class="bg-primary text-primary-foreground hover:bg-primary/90 border border-primary/50".to_string()
                >
                    "Hold to confirm"
                </ButtonAction>
                <Show when=move || fired.get()>
                    <p class="text-sm text-success">"Action fired!"</p>
                </Show>
                <Show when=move || fired.get()>
                    <button
                        class="text-xs text-muted-foreground underline"
                        on:click=move |_| fired.set(false)
                    >
                        "Reset"
                    </button>
                </Show>
            </div>
        </PageShell>
    }
}
