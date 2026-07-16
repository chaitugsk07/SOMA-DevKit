use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());

    view! {
        <div class="max-w-2xl">
            <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().home_title}</h1>
            <p class="text-sm text-muted-foreground mt-1">{move || s().home_subtitle}</p>

            <div class="mt-8 grid gap-4">
                <div class="bg-card border border-border rounded-md p-6">
                    <h2 class="text-lg font-semibold text-foreground mb-1">{move || s().home_install_heading}</h2>
                    <p class="text-sm text-muted-foreground mb-3">"Copy components directly into your Leptos project."</p>
                    <div class="bg-background rounded-md p-4 font-mono text-xs text-foreground border border-border">
                        "soma-ui = { path = \"../packages/ui\" }"
                    </div>
                </div>

                <div class="bg-card border border-border rounded-md p-6">
                    <h2 class="text-lg font-semibold text-foreground mb-3">{move || s().home_components_heading}</h2>

                    <div class="mt-0">
                        <p class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest mb-2">{move || s().nav_inputs}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                            <A href="/components/button" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_button}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_button_desc}</span>
                            </A>
                            <A href="/components/input" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_input}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_input_desc}</span>
                            </A>
                        </div>
                    </div>

                    <div class="mt-5">
                        <p class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest mb-2">{move || s().nav_data_display}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                            <A href="/components/badge" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_badge}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_badge_desc}</span>
                            </A>
                            <A href="/components/card" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_card}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_card_desc}</span>
                            </A>
                        </div>
                    </div>

                    <div class="mt-5">
                        <p class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest mb-2">{move || s().nav_layout}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                            <A href="/components/separator" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_separator}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_separator_desc}</span>
                            </A>
                        </div>
                    </div>

                    <div class="mt-5">
                        <p class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest mb-2">{move || s().nav_motion}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                            <A href="/components/animate" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_animate}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_animate_desc}</span>
                            </A>
                        </div>
                    </div>

                    <div class="mt-5">
                        <p class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest mb-2">{move || s().nav_charts}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                            <A href="/charts/area" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_area_chart}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_area_chart_desc}</span>
                            </A>
                            <A href="/charts/bar" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_bar_chart}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_bar_chart_desc}</span>
                            </A>
                            <A href="/charts/line" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_line_chart}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_line_chart_desc}</span>
                            </A>
                            <A href="/charts/pie" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_pie_chart}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_pie_chart_desc}</span>
                            </A>
                            <A href="/charts/radar" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_radar_chart}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_radar_chart_desc}</span>
                            </A>
                            <A href="/charts/radial" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_radial_chart}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_radial_chart_desc}</span>
                            </A>
                        </div>
                    </div>

                    <div class="mt-5">
                        <p class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest mb-2">{move || s().home_blocks_heading}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                            <A href="/blocks/login" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_login_block}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_login_block_desc}</span>
                            </A>
                            <A href="/blocks/sidenav" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_sidenav_block}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_sidenav_block_desc}</span>
                            </A>
                            <A href="/blocks/headers" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_header_block}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_header_block_desc}</span>
                            </A>
                            <A href="/blocks/footers" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_footer_block}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_footer_block_desc}</span>
                            </A>
                            <A href="/blocks/faq" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_faq_block}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_faq_block_desc}</span>
                            </A>
                            <A href="/blocks/integrations" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_integrations_block}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_integrations_block_desc}</span>
                            </A>
                        </div>
                    </div>

                    <div class="mt-5">
                        <p class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest mb-2">{move || s().nav_screens}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                            <A href="/screens/login" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_login_screen}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_login_screen_desc}</span>
                            </A>
                            <A href="/screens/dashboard" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_dashboard_screen}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_dashboard_screen_desc}</span>
                            </A>
                            <A href="/screens/mobile" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_mobile_screen}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_mobile_screen_desc}</span>
                            </A>
                            <A href="/screens/settings" attr:class="flex flex-col gap-1 p-3 rounded-md bg-secondary hover:bg-accent hover:text-accent-foreground transition-colors text-sm text-foreground">
                                <span class="font-medium">{move || s().home_card_settings_screen}</span>
                                <span class="text-xs text-muted-foreground">{move || s().home_card_settings_screen_desc}</span>
                            </A>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
