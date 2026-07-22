use super::shared::{EscapeBackdrop, OverlayDescription, OverlayFooter, OverlayTitle};
use crate::icons::{icondata, Icon};
use leptos::portal::Portal;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

/// Slide-over panel from any edge. Caller owns the open signal.
#[component]
pub fn Sheet(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    children: ChildrenFn,
) -> impl IntoView {
    provide_context(open);
    let side = StoredValue::new(side);
    provide_context(side);

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
pub fn SheetContent(children: Children) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("SheetContent must be inside Sheet");
    let side = use_context::<StoredValue<SheetSide>>().expect("SheetContent must be inside Sheet");
    let close = move |_| open.set(false);

    let panel_class = move || {
        let base =
            "fixed z-50 bg-card text-card-foreground border-border flex flex-col shadow-elev-lg";
        match side.get_value() {
            SheetSide::Right => {
                format!("{base} inset-y-0 end-0 h-full w-80 border-s animate-slide-left")
            }
            SheetSide::Left => {
                format!("{base} inset-y-0 start-0 h-full w-80 border-e animate-slide-right")
            }
            SheetSide::Top => {
                format!("{base} inset-x-0 top-0 w-full max-h-[50vh] border-b animate-slide-down")
            }
            SheetSide::Bottom => {
                format!("{base} inset-x-0 bottom-0 w-full max-h-[50vh] border-t animate-slide-up")
            }
        }
    };

    view! {
        <div class=panel_class>
            <div class="relative flex-1 overflow-auto p-6">
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

/// SheetHeader has `pe-8` (padding for the close button) — intentionally distinct from DialogHeader.
#[component]
pub fn SheetHeader(children: Children) -> impl IntoView {
    view! { <div class="mb-4 pe-8">{children()}</div> }
}

#[component]
pub fn SheetTitle(children: Children) -> impl IntoView {
    view! { <OverlayTitle>{children()}</OverlayTitle> }
}

#[component]
pub fn SheetDescription(children: Children) -> impl IntoView {
    view! { <OverlayDescription>{children()}</OverlayDescription> }
}

#[component]
pub fn SheetFooter(children: Children) -> impl IntoView {
    view! { <OverlayFooter>{children()}</OverlayFooter> }
}
