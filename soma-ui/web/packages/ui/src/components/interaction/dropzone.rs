use crate::components::shared::CONTROL_MOTION;
use crate::icons::{icondata, Icon};
use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;

// ponytail: file names-only via js_sys::Reflect (no extra web-sys features needed).
// DataTransfer/FileList are not in the transitive web-sys feature set, so we
// walk ev.data_transfer.files via js reflection.
// Ceiling: reading file contents (FileReader / stream API) = upgrade path.

fn extract_file_names(ev: &leptos::ev::DragEvent) -> Vec<String> {
    #[allow(unused_imports)]
    use leptos::wasm_bindgen::JsCast;
    let ev: &web_sys::DragEvent = ev.unchecked_ref();
    let dt = match js_sys::Reflect::get(ev, &"dataTransfer".into()).ok() {
        Some(v) if !v.is_null() && !v.is_undefined() => v,
        _ => return vec![],
    };
    let files = match js_sys::Reflect::get(&dt, &"files".into()).ok() {
        Some(v) if !v.is_null() && !v.is_undefined() => v,
        _ => return vec![],
    };
    let length = js_sys::Reflect::get(&files, &"length".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as u32;

    (0..length)
        .filter_map(|i| {
            let file = js_sys::Reflect::get_u32(&files, i).ok()?;
            js_sys::Reflect::get(&file, &"name".into())
                .ok()
                .and_then(|v| v.as_string())
        })
        .collect()
}

fn extract_input_file_names(ev: &leptos::ev::Event) -> Vec<String> {
    #[allow(unused_imports)]
    use leptos::wasm_bindgen::JsCast;
    let input: web_sys::HtmlInputElement = ev
        .target()
        .and_then(|t| t.dyn_into().ok())
        .unwrap_or_else(|| panic!("dropzone input event on non-input"));
    let files = match js_sys::Reflect::get(&input, &"files".into()).ok() {
        Some(v) if !v.is_null() && !v.is_undefined() => v,
        _ => return vec![],
    };
    let length = js_sys::Reflect::get(&files, &"length".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as u32;

    (0..length)
        .filter_map(|i| {
            let file = js_sys::Reflect::get_u32(&files, i).ok()?;
            js_sys::Reflect::get(&file, &"name".into())
                .ok()
                .and_then(|v| v.as_string())
        })
        .collect()
}

#[component]
pub fn Dropzone(
    /// Called with the list of dropped/selected file names.
    #[prop(optional)]
    on_files: Option<Callback<Vec<String>>>,
    /// Content shown inside the drop area (default: upload icon + hint text).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let hovering = RwSignal::new(false);
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    let on_dragover = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        hovering.set(true);
    };
    let on_dragleave = move |_: leptos::ev::DragEvent| hovering.set(false);

    let on_drop = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        hovering.set(false);
        let names = extract_file_names(&ev);
        if let Some(cb) = on_files {
            cb.run(names);
        }
    };

    let on_click = move |_| {
        if let Some(el) = input_ref.get() {
            el.click();
        }
    };

    let on_change = move |ev: leptos::ev::Event| {
        let names = extract_input_file_names(&ev);
        if let Some(cb) = on_files {
            cb.run(names);
        }
    };

    view! {
        <div
            class=move || {
                let base = format!("relative border-2 border-dashed rounded-lg p-8 text-center cursor-pointer {}", CONTROL_MOTION);
                if hovering.get() {
                    format!("{base} border-primary bg-accent/30")
                } else {
                    format!("{base} border-border hover:border-primary/70 hover:bg-accent/10")
                }
            }
            on:dragover=on_dragover
            on:dragleave=on_dragleave
            on:drop=on_drop
            on:click=on_click
        >
            // Hidden file input
            <input
                node_ref=input_ref
                type="file"
                multiple
                class="sr-only"
                on:change=on_change
            />

            // Inner content (slot or default)
            {if let Some(c) = children {
                c().into_any()
            } else {
                view! {
                    <div class="flex flex-col items-center gap-3 text-muted-foreground">
                        <Icon icon=Signal::derive(|| icondata::LuUpload) width="32" height="32" />
                        <div>
                            <p class="text-sm font-medium text-foreground">"Drag files here or click to browse"</p>
                            <p class="text-xs mt-1">"Any file type accepted"</p>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
