use super::shared::OverlayFooter;
use crate::{Button, ButtonVariant};
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;

static NEXT_ALERT_DIALOG_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
struct AlertDialogA11y {
    title_id: String,
    description_id: String,
}

/// Confirmation dialog — backdrop click does NOT dismiss; requires explicit button action.
/// ponytail: No Escape key, no backdrop dismiss — intentional semantic difference from Dialog.
#[component]
pub fn AlertDialog(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    provide_context(open);
    let id = NEXT_ALERT_DIALOG_ID.fetch_add(1, Ordering::Relaxed);
    provide_context(AlertDialogA11y {
        title_id: format!("soma-alert-dialog-title-{id}"),
        description_id: format!("soma-alert-dialog-description-{id}"),
    });

    let children = StoredValue::new(children);
    let previous_focus = StoredValue::new_local(None::<leptos::web_sys::HtmlElement>);

    Effect::new(move |was_open: Option<bool>| {
        let is_open = open.get();
        if is_open && !was_open.unwrap_or(false) {
            previous_focus.set_value(
                document()
                    .active_element()
                    .and_then(|element| element.dyn_into().ok()),
            );
        } else if !is_open && was_open.unwrap_or(false) {
            previous_focus.with_value(|element| {
                if let Some(element) = element {
                    let _ = element.focus();
                }
            });
        }
        is_open
    });

    view! {
        <Portal>
            <Show when=move || open.get()>
                // Backdrop — pointer-events-none so clicks pass through (no dismiss on click).
                <div class="fixed inset-0 z-40 bg-black/60 animate-fade-in pointer-events-none" />
                {children.with_value(|c| c())}
            </Show>
        </Portal>
    }
}

#[component]
pub fn AlertDialogContent(children: Children) -> impl IntoView {
    let a11y =
        use_context::<AlertDialogA11y>().expect("AlertDialogContent must be inside AlertDialog");
    let panel_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {
        if let Some(panel) = panel_ref.get() {
            let _ = panel.focus();
        }
    });

    view! {
        <div
            node_ref=panel_ref
            role="alertdialog"
            aria-modal="true"
            aria-labelledby=a11y.title_id
            aria-describedby=a11y.description_id
            tabindex="-1"
            class="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-50 w-full max-w-lg rounded-lg border border-border bg-card p-6 shadow-elev-lg animate-scale-in focus:outline-none"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogHeader(children: Children) -> impl IntoView {
    view! { <div class="mb-4">{children()}</div> }
}

#[component]
pub fn AlertDialogTitle(children: Children) -> impl IntoView {
    let a11y =
        use_context::<AlertDialogA11y>().expect("AlertDialogTitle must be inside AlertDialog");
    view! {
        <h2 id=a11y.title_id class="text-lg font-semibold text-foreground">{children()}</h2>
    }
}

#[component]
pub fn AlertDialogDescription(children: Children) -> impl IntoView {
    let a11y = use_context::<AlertDialogA11y>()
        .expect("AlertDialogDescription must be inside AlertDialog");
    view! {
        <p id=a11y.description_id class="text-sm text-muted-foreground mt-1">{children()}</p>
    }
}

#[component]
pub fn AlertDialogFooter(children: Children) -> impl IntoView {
    view! { <OverlayFooter>{children()}</OverlayFooter> }
}

/// Primary action button — closes and fires the callback.
#[component]
pub fn AlertDialogAction(
    #[prop(into)] on_click: Callback<()>,
    children: Children,
) -> impl IntoView {
    let open =
        use_context::<RwSignal<bool>>().expect("AlertDialogAction must be inside AlertDialog");
    view! {
        <Button
            variant=ButtonVariant::Default
            on:click=move |_| { open.set(false); on_click.run(()); }
        >
            {children()}
        </Button>
    }
}

/// Cancel button — closes without confirming.
#[component]
pub fn AlertDialogCancel(children: Children) -> impl IntoView {
    let open =
        use_context::<RwSignal<bool>>().expect("AlertDialogCancel must be inside AlertDialog");
    view! {
        <Button
            variant=ButtonVariant::Outline
            on:click=move |_| open.set(false)
        >
            {children()}
        </Button>
    }
}
