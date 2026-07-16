use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{Timeline, TimelineItem};

#[component]
pub fn TimelinePage() -> impl IntoView {
    view! {
        <PageShell
            title=Signal::derive(|| "Timeline".to_string())
            subtitle=Signal::derive(|| "Vertical timeline with status markers.".to_string())
        >
            <PreviewPanel>
                <Timeline class="w-full max-w-sm".to_string()>
                    <TimelineItem marker="1".to_string() title="Schema created".to_string() status="success".to_string()>
                        "20260101_01_init.sql applied in 42 ms"
                    </TimelineItem>
                    <TimelineItem marker="2".to_string() title="Seed data".to_string() status="applied".to_string()>
                        "20260101_02_seed.sql applied in 18 ms"
                    </TimelineItem>
                    <TimelineItem marker="3".to_string() title="Pending migration".to_string() status="pending".to_string()>
                        "20260201_01_add_index.sql not yet applied"
                    </TimelineItem>
                </Timeline>
            </PreviewPanel>
        </PageShell>
    }
}
