use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    Carousel, CarouselContent, CarouselDots, CarouselItem, CarouselNext, CarouselPrevious,
};

#[component]
pub fn CarouselPage() -> impl IntoView {
    let slides: &'static [(&'static str, &'static str, &'static str)] = &[
        ("bg-blue-900/60", "Slide 1", "Deep blue"),
        ("bg-violet-900/60", "Slide 2", "Violet night"),
        ("bg-emerald-900/60", "Slide 3", "Emerald glow"),
        ("bg-rose-900/60", "Slide 4", "Rose ember"),
    ];

    let show_dots = RwSignal::new(true);
    let show_arrows = RwSignal::new(true);

    view! {
        <PageShell
            title=Signal::derive(move || "Carousel".to_string())
            subtitle=Signal::derive(move || "One-per-view slide carousel with prev/next controls and dot indicators.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6">
                {move || view! {
                    <Carousel count=4>
                        <CarouselContent>
                            {slides.iter().map(|(bg, title, sub)| view! {
                                <CarouselItem>
                                    <div class=format!("flex flex-col items-center justify-center h-48 rounded-md {bg}")>
                                        <p class="text-lg font-semibold text-white">{*title}</p>
                                        <p class="text-sm text-white/70">{*sub}</p>
                                    </div>
                                </CarouselItem>
                            }).collect_view()}
                        </CarouselContent>
                        {if show_arrows.get() { Some(view! { <CarouselPrevious /> }) } else { None }}
                        {if show_arrows.get() { Some(view! { <CarouselNext /> }) } else { None }}
                        {if show_dots.get() { Some(view! { <CarouselDots /> }) } else { None }}
                    </Carousel>
                }}
            </div>

            <ControlsPanel>
                <ControlRow label="Show arrows">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_arrows.get()
                        on:change=move |e| show_arrows.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Show dot indicators">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_dots.get()
                        on:change=move |e| show_dots.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
