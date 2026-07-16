use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{CardCarousel, CardCarouselItem};

fn parse_card_count(s: &str) -> usize {
    s.parse().unwrap_or(8)
}

#[component]
pub fn CardCarouselPage() -> impl IntoView {
    let cards: &'static [(&'static str, &'static str)] = &[
        ("Mountains", "Majestic peaks towering over misty valleys."),
        ("Ocean", "Endless blue horizon meets the sky."),
        ("Forest", "Ancient trees whisper in the breeze."),
        ("Desert", "Golden dunes under a blazing sun."),
        ("Tundra", "Frozen plains stretch to infinity."),
        ("Savanna", "Grasslands where wildlife roams free."),
        ("Jungle", "Dense canopy alive with colour."),
        ("Coast", "Rugged cliffs carved by the waves."),
    ];

    let card_count = RwSignal::new(8usize);

    view! {
        <PageShell
            title=Signal::derive(move || "Card Carousel".to_string())
            subtitle=Signal::derive(move || "Horizontal scroll-snap card row with native CSS — no JS index tracking needed.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6">
                {move || {
                    let n = card_count.get();
                    view! {
                        <CardCarousel>
                            {cards.iter().take(n).map(|(title, desc)| view! {
                                <CardCarouselItem>
                                    <div class="p-4">
                                        <div class="h-32 bg-muted rounded-md mb-3 flex items-center justify-center text-2xl text-muted-foreground">
                                            "🏞"
                                        </div>
                                        <p class="font-medium text-sm text-foreground">{*title}</p>
                                        <p class="text-xs text-muted-foreground mt-1">{*desc}</p>
                                    </div>
                                </CardCarouselItem>
                            }).collect_view()}
                        </CardCarousel>
                    }
                }}
            </div>

            <ControlsPanel>
                <ControlRow label="Number of cards">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| card_count.set(parse_card_count(&event_target_value(&e)))
                    >
                        <option value="2">"2"</option>
                        <option value="4">"4"</option>
                        <option value="6">"6"</option>
                        <option value="8" selected>"8 (all)"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
