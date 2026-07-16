use crate::Label;
use leptos::prelude::*;

// ponytail: Field = label + control + description + error in a vertical stack.
// Manual composition via FieldLabel/FieldDescription/FieldError is the upgrade path
// when callers need non-standard layouts.

#[component]
pub fn Field(
    #[prop(into)] label: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<Signal<Option<String>>>,
    #[prop(optional)] for_id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="space-y-1.5">
            <Label for_id=for_id.unwrap_or_default()>{label}</Label>
            {children()}
            {description.map(|d| view! {
                <p class="text-xs text-muted-foreground">{d}</p>
            })}
            {error.map(|e| view! {
                <Show when=move || e.get().is_some()>
                    <p class="text-sm text-destructive">{move || e.get().unwrap_or_default()}</p>
                </Show>
            })}
        </div>
    }
}

// Thin wrappers for manual composition
#[component]
pub fn FieldLabel(
    #[prop(optional, into)] for_id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! { <Label for_id=for_id.unwrap_or_default()>{children()}</Label> }
}

#[component]
pub fn FieldDescription(children: Children) -> impl IntoView {
    view! { <p class="text-xs text-muted-foreground">{children()}</p> }
}

#[component]
pub fn FieldError(children: Children) -> impl IntoView {
    view! { <p class="text-sm text-destructive">{children()}</p> }
}
