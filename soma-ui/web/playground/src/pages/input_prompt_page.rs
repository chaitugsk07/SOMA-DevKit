use crate::ui::*;
use leptos::prelude::*;
use soma_ui::InputPrompt;

#[component]
pub fn InputPromptPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let messages: RwSignal<Vec<String>> = RwSignal::new(Vec::new());

    let on_submit = Callback::new(move |msg: String| {
        messages.update(|v| v.push(msg));
    });

    view! {
        <PageShell
            title=Signal::derive(move || "Input Prompt".to_string())
            subtitle=Signal::derive(move || "Auto-growing chat textarea. Enter sends, Shift+Enter newlines. Attach button optional.".to_string())
        >
            // Preview (bespoke: flex-col gap-4 min-h-72 with message history)
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col gap-4 min-h-72">
                // Message history
                <div class="flex-1 space-y-2 min-h-32">
                    {move || {
                        let msgs = messages.get();
                        if msgs.is_empty() {
                            view! {
                                <p class="text-sm text-muted-foreground italic">"Submitted messages appear here…"</p>
                            }.into_any()
                        } else {
                            msgs.into_iter().map(|msg| view! {
                                <div class="rounded-lg bg-zinc-800 px-3 py-2 text-sm text-foreground max-w-xs ms-auto">
                                    {msg}
                                </div>
                            }).collect::<Vec<_>>().into_any()
                        }
                    }}
                </div>

                // Prompt input with attach button visible
                <InputPrompt value=value on_submit=on_submit show_attach=true />
            </div>
        </PageShell>
    }
}
