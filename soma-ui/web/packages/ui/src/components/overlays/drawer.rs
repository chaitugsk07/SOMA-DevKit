use super::shared::{EscapeBackdrop, OverlayDescription, OverlayFooter, OverlayTitle};
use leptos::portal::Portal;
use leptos::prelude::*;

/// Bottom sheet / drawer. Caller owns the open signal.
#[component]
pub fn Drawer(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    provide_context(open);

    let children = StoredValue::new(children);

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
pub fn DrawerContent(children: Children) -> impl IntoView {
    view! {
        <div class="fixed inset-x-0 bottom-0 z-50 rounded-t-xl border-t border-border bg-card p-6 shadow-elev-lg animate-slide-up max-h-[85vh] overflow-auto">
            <div class="mx-auto mb-4 h-1.5 w-12 rounded-full bg-muted" />
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerHeader(children: Children) -> impl IntoView {
    view! { <div class="mb-4">{children()}</div> }
}

#[component]
pub fn DrawerTitle(children: Children) -> impl IntoView {
    view! { <OverlayTitle>{children()}</OverlayTitle> }
}

#[component]
pub fn DrawerDescription(children: Children) -> impl IntoView {
    view! { <OverlayDescription>{children()}</OverlayDescription> }
}

#[component]
pub fn DrawerFooter(children: Children) -> impl IntoView {
    view! { <OverlayFooter>{children()}</OverlayFooter> }
}
