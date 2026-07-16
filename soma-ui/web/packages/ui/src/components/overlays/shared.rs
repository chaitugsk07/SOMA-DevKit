use leptos::prelude::*;

/// Private backdrop+Escape helper shared by DrawerOverlay, SheetOverlay, and DialogOverlay.
///
/// Registers a keydown listener for Escape (auto-removed on unmount), renders a click-to-close
/// backdrop, then renders `children` on top.
#[component]
pub(crate) fn EscapeBackdrop(open: RwSignal<bool>, children: Children) -> impl IntoView {
    window_event_listener(leptos::ev::keydown, move |e| {
        if e.key() == "Escape" {
            open.set(false);
        }
    });

    let close = move |_| open.set(false);

    view! {
        <>
            <div class="fixed inset-0 z-40 bg-black/60 animate-fade-in" on:click=close />
            {children()}
        </>
    }
}

/// Shared title: `<h2 class="text-lg font-semibold text-foreground">`.
/// Used by Dialog*, AlertDialog*, Drawer*, Sheet* title wrappers.
#[component]
pub(crate) fn OverlayTitle(children: Children) -> impl IntoView {
    view! { <h2 class="text-lg font-semibold text-foreground">{children()}</h2> }
}

/// Shared description: `<p class="text-sm text-muted-foreground mt-1">`.
/// Used by Dialog*, AlertDialog*, Drawer*, Sheet* description wrappers.
#[component]
pub(crate) fn OverlayDescription(children: Children) -> impl IntoView {
    view! { <p class="text-sm text-muted-foreground mt-1">{children()}</p> }
}

/// Shared footer: `<div class="flex justify-end gap-2 mt-4">`.
/// Used by Dialog*, AlertDialog*, Drawer*, Sheet* footer wrappers.
#[component]
pub(crate) fn OverlayFooter(children: Children) -> impl IntoView {
    view! { <div class="flex justify-end gap-2 mt-4">{children()}</div> }
}
