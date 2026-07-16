use leptos::prelude::*;

struct Preset {
    label: &'static str,
    width: Option<u32>,
}

const PRESETS: &[Preset] = &[
    Preset {
        label: "Mobile 375",
        width: Some(375),
    },
    Preset {
        label: "Tablet 768",
        width: Some(768),
    },
    Preset {
        label: "Laptop 1024",
        width: Some(1024),
    },
    Preset {
        label: "Desktop 1440",
        width: Some(1440),
    },
    Preset {
        label: "Full",
        width: None,
    },
];

#[component]
pub fn ViewportCanvas() -> impl IntoView {
    let width = RwSignal::new(None::<u32>);

    let iframe_src = web_sys::window()
        .and_then(|w| w.location().href().ok())
        .unwrap_or_else(|| "/".to_string());

    view! {
        <div class="h-screen flex flex-col bg-background">
            // Toolbar
            <div class="flex items-center gap-2 p-3 border-b border-border bg-card">
                <span class="text-sm font-medium">"Viewport"</span>
                {PRESETS.iter().map(|preset| {
                    let preset_width = preset.width;
                    let label = preset.label;
                    view! {
                        <button
                            class=move || {
                                let base = "text-sm px-3 py-1.5 rounded-md transition-colors";
                                if width.get() == preset_width {
                                    format!("{} bg-accent text-foreground", base)
                                } else {
                                    format!("{} text-muted-foreground hover:text-foreground hover:bg-accent", base)
                                }
                            }
                            on:click=move |_| width.set(preset_width)
                        >
                            {label}
                        </button>
                    }
                }).collect_view()}
                <span class="ms-auto text-sm text-muted-foreground">
                    {move || match width.get() {
                        Some(w) => format!("{} px", w),
                        None => "Full".to_string(),
                    }}
                </span>
            </div>
            // Canvas
            <div class="flex-1 overflow-auto bg-muted/40 flex justify-center items-start p-4">
                <iframe
                    src=iframe_src
                    style=move || match width.get() {
                        Some(w) => format!("width:{}px;height:100%;", w),
                        None => "width:100%;height:100%;".to_string(),
                    }
                    class="bg-background border border-border rounded-md shadow-lg shrink-0 transition-[width] duration-200"
                    title="Component Preview"
                />
            </div>
        </div>
    }
}
