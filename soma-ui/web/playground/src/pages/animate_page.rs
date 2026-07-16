use crate::i18n::{strings, Locale};
use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Animate, AnimationType};

fn parse_animation(s: &str) -> AnimationType {
    match s {
        "SlideUp" => AnimationType::SlideUp,
        "SlideDown" => AnimationType::SlideDown,
        "SlideLeft" => AnimationType::SlideLeft,
        "SlideRight" => AnimationType::SlideRight,
        "ScaleIn" => AnimationType::ScaleIn,
        "BounceIn" => AnimationType::BounceIn,
        "PulseSoft" => AnimationType::PulseSoft,
        _ => AnimationType::FadeIn,
    }
}

#[component]
pub fn AnimatePage() -> impl IntoView {
    let animation = RwSignal::new(AnimationType::FadeIn);
    let key = RwSignal::new(0u32);
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <PageShell
            title=Signal::derive(move || s().animate_title)
            subtitle=Signal::derive(move || s().animate_subtitle)
        >
            // Preview
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex items-center justify-center min-h-52">
                {move || {
                    let _k = key.get();
                    let anim = animation.get();
                    view! {
                        <Animate animation=anim>
                            <div class="bg-primary text-primary-foreground px-6 py-3 rounded-md font-medium">
                                "Animated Element"
                            </div>
                        </Animate>
                    }
                }}
            </div>

            // Controls
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().controls}</h2>
                <div>
                    <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3 border-b border-border">
                        <span class="text-sm text-muted-foreground">{move || s().label_animation}</span>
                        <select
                            class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                            on:change=move |e| {
                                animation.set(parse_animation(&event_target_value(&e)));
                                key.update(|k| *k += 1);
                            }
                        >
                            <option value="FadeIn">"Fade In"</option>
                            <option value="SlideUp">"Slide Up"</option>
                            <option value="SlideDown">"Slide Down"</option>
                            <option value="SlideLeft">"Slide Left"</option>
                            <option value="SlideRight">"Slide Right"</option>
                            <option value="ScaleIn">"Scale In"</option>
                            <option value="BounceIn">"Bounce In"</option>
                            <option value="PulseSoft">"Pulse Soft"</option>
                        </select>
                    </div>
                    <div class="flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between py-3">
                        <span class="text-sm text-muted-foreground">{move || s().replay}</span>
                        <button
                            class="bg-secondary hover:bg-accent text-foreground text-sm px-4 py-1.5 rounded-md transition-colors"
                            on:click=move |_| key.update(|k| *k += 1)
                        >
                            {move || s().replay}
                        </button>
                    </div>
                </div>
            </div>

            // All animations
            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-4">{move || s().all_animations}</h2>
                <div class="grid grid-cols-2 gap-4">
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::FadeIn>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Fade In"</div>
                        </Animate>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::SlideUp>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Slide Up"</div>
                        </Animate>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::SlideDown>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Slide Down"</div>
                        </Animate>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::SlideLeft>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Slide Left"</div>
                        </Animate>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::SlideRight>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Slide Right"</div>
                        </Animate>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::ScaleIn>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Scale In"</div>
                        </Animate>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::BounceIn>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Bounce In"</div>
                        </Animate>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Animate animation=AnimationType::PulseSoft>
                            <div class="bg-secondary text-foreground text-xs px-3 py-2 rounded">"Pulse Soft"</div>
                        </Animate>
                    </div>
                </div>
            </div>
        </PageShell>
    }
}
