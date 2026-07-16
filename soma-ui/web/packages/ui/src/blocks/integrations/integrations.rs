use crate::components::data_display::badge::{Badge, BadgeVariant};
use crate::components::data_display::card::{Card, CardContent};
use crate::components::inputs::button::{Button, ButtonSize, ButtonVariant};
use leptos::prelude::*;

struct Integration {
    initial: &'static str,
    name: &'static str,
    description: &'static str,
    connected: bool,
}

#[component]
pub fn IntegrationsBlock() -> impl IntoView {
    let integrations = vec![
        Integration {
            initial: "GH",
            name: "GitHub",
            description: "Connect your repositories and automate workflows.",
            connected: true,
        },
        Integration {
            initial: "SL",
            name: "Slack",
            description: "Send notifications and alerts to Slack channels.",
            connected: true,
        },
        Integration {
            initial: "PG",
            name: "PostgreSQL",
            description: "Read and write data to your Postgres database.",
            connected: false,
        },
        Integration {
            initial: "S3",
            name: "AWS S3",
            description: "Store and retrieve files from S3 buckets.",
            connected: false,
        },
        Integration {
            initial: "ST",
            name: "Stripe",
            description: "Process payments and manage subscriptions.",
            connected: false,
        },
        Integration {
            initial: "DS",
            name: "Datadog",
            description: "Monitor your application metrics and logs.",
            connected: false,
        },
    ];

    view! {
        <div class="p-8 space-y-6">
            <div>
                <h2 class="font-heading text-2xl font-bold text-foreground">"Integrations"</h2>
                <p class="text-sm text-muted-foreground mt-1">"Connect your favourite tools and services."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {integrations.into_iter().map(|i| {
                    view! {
                        <Card>
                            <CardContent class="pt-6".to_string()>
                                <div class="flex items-start justify-between mb-3">
                                    <div class="h-10 w-10 rounded-md bg-muted flex items-center justify-center text-sm font-semibold text-foreground">
                                        {i.initial}
                                    </div>
                                    {if i.connected {
                                        view! { <Badge variant=BadgeVariant::Success>"Connected"</Badge> }.into_any()
                                    } else {
                                        view! { <span /> }.into_any()
                                    }}
                                </div>
                                <h3 class="font-medium text-foreground text-sm">{i.name}</h3>
                                <p class="text-xs text-muted-foreground mt-0.5 mb-4">{i.description}</p>
                                <Button
                                    variant=if i.connected { ButtonVariant::Outline } else { ButtonVariant::Default }
                                    size=ButtonSize::Sm
                                    class="w-full".to_string()
                                >
                                    {if i.connected { "Disconnect" } else { "Connect" }}
                                </Button>
                            </CardContent>
                        </Card>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
