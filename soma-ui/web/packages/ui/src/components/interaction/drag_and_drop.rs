use crate::components::shared::CONTROL_MOTION;
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

// ponytail: index-based native HTML5 DnD reorder.
// Drag source index stored in RwSignal; on drop into target row, splice Vec.
// Ceiling: touch devices, cross-list drag, keyboard reorder = upgrade path.

#[component]
pub fn DragAndDrop(
    /// The list of items to reorder. Updated in-place on drop.
    #[prop(into)]
    items: RwSignal<Vec<String>>,
) -> impl IntoView {
    let drag_src: RwSignal<Option<usize>> = RwSignal::new(None);
    let drag_over: RwSignal<Option<usize>> = RwSignal::new(None);

    view! {
        <div class="flex flex-col gap-1">
            {move || {
                items.get().into_iter().enumerate().map(|(i, item)| {
                    let item = StoredValue::new(item);
                    view! {
                        <div
                            draggable="true"
                            class=move || {
                                let base = format!("flex items-center gap-2 rounded-md border border-border bg-card p-2 text-sm text-foreground cursor-grab select-none {}", CONTROL_MOTION);
                                if drag_over.get() == Some(i) && drag_src.get() != Some(i) {
                                    format!("{base} border-primary bg-accent/30")
                                } else {
                                    base.to_string()
                                }
                            }
                            on:dragstart=move |_| {
                                drag_src.set(Some(i));
                            }
                            on:dragover=move |ev| {
                                ev.prevent_default();
                                drag_over.set(Some(i));
                            }
                            on:dragleave=move |_| {
                                if drag_over.get() == Some(i) {
                                    drag_over.set(None);
                                }
                            }
                            on:drop=move |ev| {
                                ev.prevent_default();
                                if let Some(src) = drag_src.get() {
                                    if src != i {
                                        items.update(|v| {
                                            let removed = v.remove(src);
                                            let target = if src < i { i } else { i };
                                            v.insert(target, removed);
                                        });
                                    }
                                }
                                drag_src.set(None);
                                drag_over.set(None);
                            }
                            on:dragend=move |_| {
                                drag_src.set(None);
                                drag_over.set(None);
                            }
                        >
                            <span class="text-muted-foreground">
                                <Icon icon=Signal::derive(|| icondata::LuGripVertical) width="16" height="16" />
                            </span>
                            {item.get_value()}
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}
