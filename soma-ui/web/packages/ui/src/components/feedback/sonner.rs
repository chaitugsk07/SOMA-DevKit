use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use leptos::leptos_dom::helpers::set_timeout;
use leptos::portal::Portal;
use leptos::prelude::*;
use std::time::Duration;

/// Sonner-style compact toast notifications with a provider + Portal viewport.
///
/// Mount `<SonnerToaster>` once near your app root. Call `use_sonner()` from
/// any descendant to get the imperative handle. The Portal viewport renders to
/// `document.body` so toasts are never clipped by overflow.
///
/// ponytail: true Sonner stacking/expand-on-hover is simplified to a flat stack.
/// Upgrade path: track indices and apply translateY offsets for the stacked-card look.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SonnerVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug)]
pub struct SonnerItem {
    pub id: usize,
    pub message: String,
    pub variant: SonnerVariant,
}

#[derive(Clone, Copy)]
pub struct SonnerHandle {
    queue: RwSignal<Vec<SonnerItem>>,
    next_id: RwSignal<usize>,
}

impl SonnerHandle {
    pub fn show(&self, message: impl Into<String>, variant: SonnerVariant, duration_ms: u64) {
        let id = self.next_id.get();
        self.next_id.update(|n| *n += 1);
        let item = SonnerItem {
            id,
            message: message.into(),
            variant,
        };
        self.queue.update(|q| q.push(item));
        let queue = self.queue;
        set_timeout(
            move || queue.update(|q| q.retain(|t| t.id != id)),
            Duration::from_millis(duration_ms),
        );
    }

    pub fn dismiss(&self, id: usize) {
        self.queue.update(|q| q.retain(|t| t.id != id));
    }
}

/// Returns the `SonnerHandle` from context. Panics if `<SonnerToaster>` is not mounted.
pub fn use_sonner() -> SonnerHandle {
    use_context::<SonnerHandle>().expect("use_sonner: <SonnerToaster> not mounted")
}

/// Provider component. Wrap your app (or a subtree) with this, then call
/// `use_sonner()` from any descendant.
#[component]
pub fn SonnerToaster(children: ChildrenFn) -> impl IntoView {
    let queue: RwSignal<Vec<SonnerItem>> = RwSignal::new(Vec::new());
    let next_id: RwSignal<usize> = RwSignal::new(0);
    let handle = SonnerHandle { queue, next_id };
    provide_context(handle);

    let children = StoredValue::new(children);

    view! {
        <>
            {children.with_value(|c| c())}
            <Portal>
                <div class="fixed bottom-4 end-4 z-[100] flex flex-col gap-2 pointer-events-none">
                    <For
                        each=move || queue.get()
                        key=|t| t.id
                        children=move |item| {
                            let id = item.id;
                            let (icon_view, text_class) = match item.variant {
                                SonnerVariant::Default => (view! {
                                    <span></span>
                                }.into_any(), "text-foreground"),
                                SonnerVariant::Success => (view! {
                                    <Icon icon=Signal::derive(|| icondata::LuCheck) width="15" height="15" attr:class="text-success shrink-0" />
                                }.into_any(), "text-foreground"),
                                SonnerVariant::Error => (view! {
                                    <Icon icon=Signal::derive(|| icondata::LuX) width="15" height="15" attr:class="text-destructive shrink-0" />
                                }.into_any(), "text-foreground"),
                                SonnerVariant::Warning => (view! {
                                    <Icon icon=Signal::derive(|| icondata::LuTriangleAlert) width="15" height="15" attr:class="text-warning shrink-0" />
                                }.into_any(), "text-foreground"),
                                SonnerVariant::Info => (view! {
                                    <Icon icon=Signal::derive(|| icondata::LuInfo) width="15" height="15" attr:class="text-info shrink-0" />
                                }.into_any(), "text-foreground"),
                            };
                            let message = item.message.clone();
                            view! {
                                <div class="pointer-events-auto flex items-center gap-2 rounded-lg border border-border bg-card px-4 py-3 shadow-elev-lg text-sm animate-slide-left min-w-64 max-w-sm">
                                    {icon_view}
                                    <span class=format!("flex-1 {}", text_class)>{message}</span>
                                    <button
                                        class=format!("flex items-center justify-center w-5 h-5 shrink-0 rounded text-muted-foreground hover:text-foreground hover:bg-accent {} {}", CONTROL_MOTION, FOCUS_RING)
                                        aria-label="Dismiss"
                                        on:click=move |_| queue.update(|q| q.retain(|t| t.id != id))
                                    >
                                        <Icon icon=Signal::derive(|| icondata::LuX) width="12" height="12" />
                                    </button>
                                </div>
                            }
                        }
                    />
                </div>
            </Portal>
        </>
    }
}
