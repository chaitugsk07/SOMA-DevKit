use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use leptos::leptos_dom::helpers::set_timeout;
use leptos::portal::Portal;
use leptos::prelude::*;
use std::time::Duration;

/// Shadcn-style toast notifications with a provider + Portal viewport.
///
/// Mount `<Toaster>` once near your app root. Call `use_toast()` from any
/// descendant to get the imperative handle. The Portal viewport renders to
/// `document.body` so toasts are never clipped by overflow.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Destructive,
    Warning,
    Info,
}

#[derive(Clone, Debug)]
pub struct ToastItem {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
}

#[derive(Clone, Copy)]
pub struct ToastHandle {
    queue: RwSignal<Vec<ToastItem>>,
    next_id: RwSignal<usize>,
}

impl ToastHandle {
    pub fn show(
        &self,
        title: impl Into<String>,
        description: Option<String>,
        variant: ToastVariant,
        duration_ms: u64,
    ) {
        // show() is called imperatively from event handlers / async tasks that are
        // not reactive tracking scopes; use get_untracked to avoid the
        // "signal accessed outside reactive context" warning (and dropped toasts).
        let id = self.next_id.get_untracked();
        self.next_id.update(|n| *n += 1);
        let item = ToastItem {
            id,
            title: title.into(),
            description,
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

/// Returns the `ToastHandle` from context. Panics if `<Toaster>` is not mounted.
pub fn use_toast() -> ToastHandle {
    use_context::<ToastHandle>().expect("use_toast: <Toaster> not mounted")
}

/// Provider component. Wrap your app (or a subtree) with this, then call
/// `use_toast()` from any descendant.
#[component]
pub fn Toaster(children: ChildrenFn) -> impl IntoView {
    let queue: RwSignal<Vec<ToastItem>> = RwSignal::new(Vec::new());
    let next_id: RwSignal<usize> = RwSignal::new(0);
    let handle = ToastHandle { queue, next_id };
    provide_context(handle);

    let children = StoredValue::new(children);

    view! {
        <>
            {children.with_value(|c| c())}
            <Portal>
                <div class="fixed bottom-4 end-4 z-[100] flex flex-col gap-2 pointer-events-none w-full max-w-sm">
                    <For
                        each=move || queue.get()
                        key=|t| t.id
                        children=move |item| {
                            let id = item.id;
                            let variant_border = match item.variant {
                                ToastVariant::Default => "border-border",
                                ToastVariant::Success => "border-success/50",
                                ToastVariant::Destructive => "border-destructive/50",
                                ToastVariant::Warning => "border-warning/50",
                                ToastVariant::Info => "border-info/50",
                            };
                            let icon_view = match item.variant {
                                ToastVariant::Default => None,
                                ToastVariant::Success => Some(view! {
                                    <Icon icon=Signal::derive(|| icondata::LuCheck) width="16" height="16" attr:class="text-success shrink-0 mt-0.5" />
                                }),
                                ToastVariant::Destructive => Some(view! {
                                    <Icon icon=Signal::derive(|| icondata::LuX) width="16" height="16" attr:class="text-destructive shrink-0 mt-0.5" />
                                }),
                                ToastVariant::Warning => Some(view! {
                                    <Icon icon=Signal::derive(|| icondata::LuTriangleAlert) width="16" height="16" attr:class="text-warning shrink-0 mt-0.5" />
                                }),
                                ToastVariant::Info => Some(view! {
                                    <Icon icon=Signal::derive(|| icondata::LuInfo) width="16" height="16" attr:class="text-info shrink-0 mt-0.5" />
                                }),
                            };
                            let class = format!(
                                "pointer-events-auto flex w-full items-start gap-3 rounded-md border {} bg-card p-4 shadow-elev-lg animate-slide-up",
                                variant_border
                            );
                            let title = item.title.clone();
                            let description = item.description.clone();
                            view! {
                                <div class=class>
                                    {icon_view}
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm font-medium text-foreground">{title}</p>
                                        {description.map(|d| view! {
                                            <p class="text-sm text-muted-foreground mt-0.5">{d}</p>
                                        })}
                                    </div>
                                    <button
                                        class=format!("flex items-center justify-center w-6 h-6 shrink-0 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent {} {}", CONTROL_MOTION, FOCUS_RING)
                                        aria-label="Dismiss"
                                        on:click=move |_| queue.update(|q| q.retain(|t| t.id != id))
                                    >
                                        <Icon icon=Signal::derive(|| icondata::LuX) width="14" height="14" />
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
