use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{FeatureCard, FeatureGrid};

#[component]
pub fn FeatureGridPage() -> impl IntoView {
    view! {
        <PageShell
            title=Signal::derive(|| "Feature Grid".to_string())
            subtitle=Signal::derive(|| "Responsive grid of feature cards for marketing pages.".to_string())
        >
            <PreviewPanel>
                <FeatureGrid>
                    <FeatureCard
                        icon="🔒".to_string()
                        title="Advisory Locking".to_string()
                        description="Postgres advisory locks prevent concurrent migrations across all replicas."
                    />
                    <FeatureCard
                        icon="🔍".to_string()
                        title="Drift Detection".to_string()
                        description="SHA-256 checksums catch any modification to previously applied migrations."
                    />
                    <FeatureCard
                        icon="📋".to_string()
                        title="Manifest Order".to_string()
                        description="migration-order.yaml gives deterministic FK-safe rollback, independent of filenames."
                    />
                </FeatureGrid>
            </PreviewPanel>
        </PageShell>
    }
}
