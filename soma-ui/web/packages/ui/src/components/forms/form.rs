use crate::Label;
use leptos::prelude::*;

// ponytail: Form = <form> wrapper with prevent_default + optional submit callback.
// State/validation lives in caller signals; Form does not own it.

#[component]
pub fn Form(
    #[prop(optional)] on_submit: Option<Callback<()>>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("space-y-6 {}", class);
    let handle = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        if let Some(cb) = on_submit {
            cb.run(());
        }
    };
    view! {
        <form class=combined on:submit=handle>
            {children()}
        </form>
    }
}

// shadcn-style sub-pieces for manual composition
#[component]
pub fn FormItem(children: Children) -> impl IntoView {
    view! { <div class="space-y-1.5">{children()}</div> }
}

#[component]
pub fn FormLabel(
    #[prop(optional, into)] for_id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! { <Label for_id=for_id.unwrap_or_default()>{children()}</Label> }
}

#[component]
pub fn FormDescription(children: Children) -> impl IntoView {
    view! { <p class="text-xs text-muted-foreground">{children()}</p> }
}

#[component]
pub fn FormMessage(children: Children) -> impl IntoView {
    view! { <p class="text-sm text-destructive">{children()}</p> }
}
