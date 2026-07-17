use crate::Checkbox;
use leptos::prelude::*;

/// A single settings-list row: checkbox + label/description + optional trailing badge.
/// Clicking anywhere on the row toggles via native label–button association.
#[component]
pub fn SettingRow(
    checked: RwSignal<bool>,
    #[prop(into)] label: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] badge: Option<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!(
        "flex min-h-[52px] cursor-pointer items-start gap-3 rounded-md border border-border p-3 {}",
        class
    );
    view! {
        <label class=combined>
            <Checkbox checked=checked />
            <div class="flex flex-1 flex-col gap-0.5">
                <span class="text-sm font-medium">{label}</span>
                {description.map(|d| view! {
                    <span class="text-xs text-muted-foreground">{d}</span>
                })}
            </div>
            {badge.map(|b| view! {
                <span class="text-[9px] uppercase tracking-wider text-muted-foreground">{b}</span>
            })}
        </label>
    }
}
