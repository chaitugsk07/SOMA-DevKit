use leptos::prelude::*;

#[derive(Clone)]
pub struct SelectableItem {
    pub id: String,
    pub label: String,
    pub sublabel: Option<String>,
    pub meta: Option<String>,
}

#[component]
pub fn SelectableList(
    items: Vec<SelectableItem>,
    selected: RwSignal<Option<String>>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!("flex flex-col overflow-y-auto {}", class);
    view! {
        <div class=combined>
            {items.into_iter().map(|item| {
                let id = item.id.clone();
                let id_click = item.id;
                view! {
                    <button
                        class=move || {
                            let active = selected.get().as_deref() == Some(id.as_str());
                            format!(
                                "flex items-start justify-between gap-2 border-b border-border p-3 text-left w-full hover:bg-muted/60{}",
                                if active { " bg-accent/30" } else { "" }
                            )
                        }
                        on:click=move |_| selected.set(Some(id_click.clone()))
                    >
                        <div class="flex flex-col items-start">
                            <span class="text-sm font-medium">{item.label}</span>
                            {item.sublabel.map(|s| view! {
                                <span class="text-xs text-muted-foreground">{s}</span>
                            })}
                        </div>
                        {item.meta.map(|m| view! {
                            <span class="text-xs text-muted-foreground">{m}</span>
                        })}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
