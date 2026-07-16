use crate::ui::*;
use leptos::prelude::*;
use soma_ui::Dropzone;

#[component]
pub fn DropzonePage() -> impl IntoView {
    let dropped_files: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    let on_files = Callback::new(move |names: Vec<String>| {
        dropped_files.set(names);
    });

    view! {
        <PageShell
            title=Signal::derive(move || "Dropzone".to_string())
            subtitle=Signal::derive(move || "Drop files or click to open a file picker. File names are shown below.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 space-y-4">
                <Dropzone on_files=on_files />

                <Show when=move || !dropped_files.get().is_empty()>
                    <div class="pt-2 border-t border-border space-y-1">
                        <p class="text-xs font-medium text-muted-foreground uppercase tracking-wider">"Selected files"</p>
                        {move || dropped_files.get().into_iter().map(|name| view! {
                            <div class="flex items-center gap-2 text-sm text-foreground py-1">
                                <span class="text-muted-foreground">"📄"</span>
                                {name}
                            </div>
                        }).collect_view()}
                    </div>
                </Show>
            </div>
        </PageShell>
    }
}
