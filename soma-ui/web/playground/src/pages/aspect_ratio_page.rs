use crate::ui::*;
use leptos::prelude::*;
use soma_ui::AspectRatio;

fn parse_ratio(s: &str) -> f64 {
    match s {
        "4:3" => 4.0 / 3.0,
        "1:1" => 1.0,
        "21:9" => 21.0 / 9.0,
        _ => 16.0 / 9.0,
    }
}

#[component]
pub fn AspectRatioPage() -> impl IntoView {
    let ratio_label = RwSignal::new("16:9".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Aspect Ratio".to_string())
            subtitle=Signal::derive(move || "Constrains content to a fixed aspect ratio.".to_string())
        >
            <PreviewPanel>
                <div class="w-80">
                    {move || {
                        let label = ratio_label.get();
                        let ratio = parse_ratio(&label);
                        view! {
                            <AspectRatio ratio=ratio>
                                <div class="w-full h-full bg-muted rounded-md flex items-center justify-center text-muted-foreground text-sm">
                                    {label}
                                </div>
                            </AspectRatio>
                        }
                    }}
                </div>
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Ratio">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| ratio_label.set(event_target_value(&e))
                    >
                        <option value="16:9" selected>"16:9"</option>
                        <option value="4:3">"4:3"</option>
                        <option value="1:1">"1:1"</option>
                        <option value="21:9">"21:9"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">"All Variants"</h2>
                <div class="flex gap-6 flex-wrap">
                    <div class="w-32">
                        <p class="text-xs text-muted-foreground mb-1">"16:9"</p>
                        <AspectRatio ratio=16.0/9.0>
                            <div class="w-full h-full bg-muted rounded-md flex items-center justify-center text-muted-foreground text-xs">"16/9"</div>
                        </AspectRatio>
                    </div>
                    <div class="w-32">
                        <p class="text-xs text-muted-foreground mb-1">"4:3"</p>
                        <AspectRatio ratio=4.0/3.0>
                            <div class="w-full h-full bg-muted rounded-md flex items-center justify-center text-muted-foreground text-xs">"4/3"</div>
                        </AspectRatio>
                    </div>
                    <div class="w-32">
                        <p class="text-xs text-muted-foreground mb-1">"1:1"</p>
                        <AspectRatio ratio=1.0>
                            <div class="w-full h-full bg-muted rounded-md flex items-center justify-center text-muted-foreground text-xs">"1/1"</div>
                        </AspectRatio>
                    </div>
                    <div class="w-32">
                        <p class="text-xs text-muted-foreground mb-1">"21:9"</p>
                        <AspectRatio ratio=21.0/9.0>
                            <div class="w-full h-full bg-muted rounded-md flex items-center justify-center text-muted-foreground text-xs">"21/9"</div>
                        </AspectRatio>
                    </div>
                </div>
            </div>
        </PageShell>
    }
}
