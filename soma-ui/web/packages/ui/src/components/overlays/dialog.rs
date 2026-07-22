use super::shared::{EscapeBackdrop, OverlayDescription, OverlayFooter};
use crate::icons::{icondata, Icon};
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;

static NEXT_DIALOG_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
struct DialogA11y {
    title_id: String,
}

/// Centered modal dialog. Caller owns the open signal.
/// ponytail: Escape-to-close via window_event_listener (leptos_dom::helpers, re-exported from
/// leptos::prelude::*). Registered in DialogOverlay — auto-removed on unmount.
#[component]
pub fn Dialog(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    provide_context(open);
    provide_context(DialogA11y {
        title_id: format!(
            "soma-dialog-title-{}",
            NEXT_DIALOG_ID.fetch_add(1, Ordering::Relaxed)
        ),
    });

    // StoredValue allows repeated access from Fn closures (unlike a plain captured variable).
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
                <EscapeBackdrop open=open>
                    {children.with_value(|c| c())}
                </EscapeBackdrop>
            </Show>
        </Portal>
    }
}

#[component]
pub fn DialogContent(children: Children) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("DialogContent must be inside Dialog");
    let a11y = use_context::<DialogA11y>().expect("DialogContent must be inside Dialog");
    let close = move |_| open.set(false);
    let panel_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {
        if let Some(panel) = panel_ref.get() {
            let _ = panel.focus();
        }
    });

    // Center via a flex wrapper, NOT top/left + -translate-*. The panel runs `animate-scale-in`
    // (`forwards`), whose final keyframe sets `transform`, which clobbered the centering translate on
    // the same element — dropping tall dialogs off-screen. Flex centering is immune to that.
    view! {
        // pointer-events-none lets clicks fall through the centering wrapper to the z-40 backdrop
        // (click-to-close); pointer-events-auto re-enables interaction on the panel itself.
        <div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
            <div
                node_ref=panel_ref
                role="dialog"
                aria-modal="true"
                aria-labelledby=a11y.title_id
                tabindex="-1"
                class="relative w-full max-w-lg max-h-[85vh] overflow-y-auto rounded-lg border border-border bg-card text-card-foreground p-6 shadow-elev-lg animate-scale-in pointer-events-auto focus:outline-none"
            >
                <button
                    class="absolute top-4 end-4 flex items-center justify-center w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors transition-all duration-150 ease-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background"
                    aria-label="Close"
                    on:click=close
                >
                    <Icon icon=Signal::derive(|| icondata::LuX) width="16" height="16" />
                </button>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn DialogHeader(children: Children) -> impl IntoView {
    view! { <div class="mb-4">{children()}</div> }
}

#[component]
pub fn DialogTitle(children: Children) -> impl IntoView {
    let a11y = use_context::<DialogA11y>().expect("DialogTitle must be inside Dialog");
    view! {
        <h2 id=a11y.title_id class="text-lg font-semibold text-foreground">{children()}</h2>
    }
}

#[component]
pub fn DialogDescription(children: Children) -> impl IntoView {
    view! { <OverlayDescription>{children()}</OverlayDescription> }
}

#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! { <OverlayFooter>{children()}</OverlayFooter> }
}
