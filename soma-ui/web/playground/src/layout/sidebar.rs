use crate::i18n::{dir, strings, Locale};
use leptos::prelude::*;
use leptos_router::components::A;

fn toggle_theme() {
    let _ = js_sys::eval(
        "const d=document.documentElement;\
         const dark=d.classList.toggle('dark');\
         localStorage.setItem('theme',dark?'dark':'light');",
    );
}

fn is_dark() -> bool {
    js_sys::eval("document.documentElement.classList.contains('dark')")
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

#[component]
fn NavSection(#[prop(into)] title: Signal<String>, children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);

    let chevron = r#"<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" fill="currentColor" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M1.646 4.646a.5.5 0 0 1 .708 0L8 10.293l5.646-5.647a.5.5 0 0 1 .708.708l-6 6a.5.5 0 0 1-.708 0l-6-6a.5.5 0 0 1 0-.708"/></svg>"#;

    view! {
        <div>
            <button
                class="font-heading text-xs font-semibold text-muted-foreground uppercase tracking-widest flex items-center justify-between w-full hover:text-foreground transition-colors mb-2"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <span>{move || title.get()}</span>
                <span
                    class=move || if open.get() {
                        "transition-transform"
                    } else {
                        "transition-transform -rotate-90"
                    }
                    inner_html=chevron
                />
            </button>
            <Show when=move || open.get()>
                <div class="flex flex-col gap-0.5">
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let dark = RwSignal::new(is_dark());
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let drawer_open = RwSignal::new(false);

    let on_toggle = move |_| {
        toggle_theme();
        dark.set(is_dark());
    };

    let moon_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16"><path d="M6 .278a.77.77 0 0 1 .08.858 7.2 7.2 0 0 0-.878 3.46c0 4.021 3.278 7.277 7.318 7.277q.792-.001 1.533-.16a.79.79 0 0 1 .81.316.73.73 0 0 1-.031.893A8.35 8.35 0 0 1 8.344 16C3.734 16 0 12.286 0 7.71 0 4.266 2.114 1.312 5.124.06A.75.75 0 0 1 6 .278"/></svg>"#;
    let sun_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16"><path d="M8 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8M8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0m0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13m8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5M3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8m10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.415a.5.5 0 1 1-.707-.708l1.414-1.414a.5.5 0 0 1 .707 0m-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0zm9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707M4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707z"/></svg>"#;

    let hamburger_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"/></svg>"#;

    let s = move || strings(locale.get());

    let nav_link_class = "block px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors aria-[current=page]:border-s-2 aria-[current=page]:border-primary aria-[current=page]:ps-[10px] aria-[current=page]:bg-accent aria-[current=page]:text-foreground";

    view! {
        <>
            // Desktop sidebar (hidden on mobile)
            <nav class="hidden md:flex flex-col bg-card border-e border-border w-60 h-screen sticky top-0 overflow-y-auto p-4 gap-6 shrink-0">
                // Header
                <div class="flex items-center gap-2">
                    <span class="font-heading text-lg font-bold text-foreground">{move || s().brand}</span>
                    <span class="bg-primary text-primary-foreground text-xs px-1.5 py-0.5 rounded font-mono">"0.1"</span>
                    <div class="ms-auto flex items-center gap-1">
                        <button
                            class="text-xs px-2 py-1 rounded hover:bg-accent text-muted-foreground hover:text-foreground transition-colors"
                            on:click=move |_| {
                                locale.set(Locale::En);
                                let _ = js_sys::eval(&format!("document.documentElement.dir='{}'", dir(Locale::En)));
                            }
                        >
                            {move || s().lang_en}
                        </button>
                        <button
                            class="text-xs px-2 py-1 rounded hover:bg-accent text-muted-foreground hover:text-foreground transition-colors"
                            on:click=move |_| {
                                locale.set(Locale::Ar);
                                let _ = js_sys::eval(&format!("document.documentElement.dir='{}'", dir(Locale::Ar)));
                            }
                        >
                            {move || s().lang_ar}
                        </button>
                        <button
                            class="flex items-center justify-center w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
                            on:click=on_toggle
                        >
                            <span inner_html=move || if dark.get() { sun_svg } else { moon_svg } />
                        </button>
                    </div>
                </div>

                // Getting Started
                <NavSection title=Signal::derive(move || s().nav_getting_started.to_string())>
                    <A href="/" attr:class=nav_link_class>{move || s().nav_home}</A>
                </NavSection>

                // Foundations
                <NavSection title=Signal::derive(move || s().nav_foundations.to_string())>
                    <A href="/theme" attr:class=nav_link_class>{move || s().nav_theme}</A>
                </NavSection>

                // Hooks
                <NavSection title=Signal::derive(move || s().nav_hooks.to_string())>
                    <A href="/hooks" attr:class=nav_link_class>{move || s().nav_hooks}</A>
                </NavSection>

                // Icons
                <NavSection title=Signal::derive(move || s().nav_icons.to_string())>
                    <A href="/icons" attr:class=nav_link_class>{move || s().nav_icons}</A>
                </NavSection>

                // Inputs
                <NavSection title=Signal::derive(move || s().nav_inputs.to_string())>
                    <A href="/components/button" attr:class=nav_link_class>{move || s().nav_button}</A>
                    <A href="/components/input" attr:class=nav_link_class>{move || s().nav_input}</A>
                    <A href="/components/textarea" attr:class=nav_link_class>{move || s().nav_textarea}</A>
                    <A href="/components/checkbox" attr:class=nav_link_class>{move || s().nav_checkbox}</A>
                    <A href="/components/switch" attr:class=nav_link_class>{move || s().nav_switch}</A>
                    <A href="/components/slider" attr:class=nav_link_class>{move || s().nav_slider}</A>
                    <A href="/components/radio-button" attr:class=nav_link_class>{move || s().nav_radio_button}</A>
                    <A href="/components/radio-button-group" attr:class=nav_link_class>{move || s().nav_radio_button_group}</A>
                    <A href="/components/toggle-group" attr:class=nav_link_class>{move || s().nav_toggle_group}</A>
                    <A href="/components/button-group" attr:class=nav_link_class>{move || s().nav_button_group}</A>
                    <A href="/components/button-action" attr:class=nav_link_class>{move || s().nav_button_action}</A>
                    <A href="/components/input-group" attr:class=nav_link_class>{move || s().nav_input_group}</A>
                    <A href="/components/select" attr:class=nav_link_class>{move || s().nav_select}</A>
                    <A href="/components/combobox" attr:class=nav_link_class>{move || s().nav_combobox}</A>
                    <A href="/components/multi-select" attr:class=nav_link_class>{move || s().nav_multi_select}</A>
                    <A href="/components/date-picker" attr:class=nav_link_class>{move || s().nav_date_picker}</A>
                    <A href="/components/input-otp" attr:class=nav_link_class>{move || s().nav_input_otp}</A>
                    <A href="/components/input-phone" attr:class=nav_link_class>{move || s().nav_input_phone}</A>
                    <A href="/components/input-prompt" attr:class=nav_link_class>{move || s().nav_input_prompt}</A>
                </NavSection>

                // Forms
                <NavSection title=Signal::derive(move || s().nav_forms.to_string())>
                    <A href="/components/label" attr:class=nav_link_class>{move || s().nav_label}</A>
                    <A href="/components/field" attr:class=nav_link_class>{move || s().nav_field}</A>
                    <A href="/components/form" attr:class=nav_link_class>{move || s().nav_form}</A>
                    <A href="/components/auto-form" attr:class=nav_link_class>{move || s().nav_auto_form}</A>
                </NavSection>

                // Data Display
                <NavSection title=Signal::derive(move || s().nav_data_display.to_string())>
                    <A href="/components/badge" attr:class=nav_link_class>{move || s().nav_badge}</A>
                    <A href="/components/card" attr:class=nav_link_class>{move || s().nav_card}</A>
                    <A href="/components/stat" attr:class=nav_link_class>{move || s().nav_stat}</A>
                    <A href="/components/avatar" attr:class=nav_link_class>{move || s().nav_avatar}</A>
                    <A href="/components/kbd" attr:class=nav_link_class>{move || s().nav_kbd}</A>
                    <A href="/components/chip" attr:class=nav_link_class>{move || s().nav_chip}</A>
                    <A href="/components/empty" attr:class=nav_link_class>{move || s().nav_empty}</A>
                    <A href="/components/item" attr:class=nav_link_class>{move || s().nav_item}</A>
                    <A href="/components/table" attr:class=nav_link_class>{move || s().nav_table}</A>
                    <A href="/components/data-table" attr:class=nav_link_class>{move || s().nav_data_table}</A>
                    <A href="/components/data-grid" attr:class=nav_link_class>{move || s().nav_data_grid}</A>
                    <A href="/components/code-block" attr:class=nav_link_class>{move || s().nav_code_block}</A>
                    <A href="/components/timeline" attr:class=nav_link_class>{move || s().nav_timeline}</A>
                    <A href="/components/comparison-table" attr:class=nav_link_class>{move || s().nav_comparison_table}</A>
                </NavSection>

                // Feedback
                <NavSection title=Signal::derive(move || s().nav_feedback.to_string())>
                    <A href="/components/skeleton" attr:class=nav_link_class>{move || s().nav_skeleton}</A>
                    <A href="/components/spinner" attr:class=nav_link_class>{move || s().nav_spinner}</A>
                    <A href="/components/alert" attr:class=nav_link_class>{move || s().nav_alert}</A>
                    <A href="/components/callout" attr:class=nav_link_class>{move || s().nav_callout}</A>
                    <A href="/components/progress" attr:class=nav_link_class>{move || s().nav_progress}</A>
                    <A href="/components/status" attr:class=nav_link_class>{move || s().nav_status}</A>
                    <A href="/components/toast" attr:class=nav_link_class>{move || s().nav_toast}</A>
                    <A href="/components/sonner" attr:class=nav_link_class>{move || s().nav_sonner}</A>
                </NavSection>

                // Layout
                <NavSection title=Signal::derive(move || s().nav_layout.to_string())>
                    <A href="/components/separator" attr:class=nav_link_class>{move || s().nav_separator}</A>
                    <A href="/components/aspect-ratio" attr:class=nav_link_class>{move || s().nav_aspect_ratio}</A>
                    <A href="/components/scroll-area" attr:class=nav_link_class>{move || s().nav_scroll_area}</A>
                    <A href="/components/page-header" attr:class=nav_link_class>{move || s().nav_page_header}</A>
                </NavSection>

                // Navigation
                <NavSection title=Signal::derive(move || s().nav_navigation.to_string())>
                    <A href="/components/breadcrumb" attr:class=nav_link_class>{move || s().nav_breadcrumb}</A>
                    <A href="/components/tabs" attr:class=nav_link_class>{move || s().nav_tabs}</A>
                    <A href="/components/pagination" attr:class=nav_link_class>{move || s().nav_pagination}</A>
                    <A href="/components/bottom-nav" attr:class=nav_link_class>{move || s().nav_bottom_nav}</A>
                    <A href="/components/navigation-menu" attr:class=nav_link_class>{move || s().nav_navigation_menu}</A>
                    <A href="/components/sidebar" attr:class=nav_link_class>{move || s().nav_sidebar}</A>
                </NavSection>

                // Disclosure
                <NavSection title=Signal::derive(move || s().nav_disclosure.to_string())>
                    <A href="/components/accordion" attr:class=nav_link_class>{move || s().nav_accordion}</A>
                    <A href="/components/collapsible" attr:class=nav_link_class>{move || s().nav_collapsible}</A>
                </NavSection>

                // Overlays
                <NavSection title=Signal::derive(move || s().nav_overlays.to_string())>
                    <A href="/components/dialog" attr:class=nav_link_class>{move || s().nav_dialog}</A>
                    <A href="/components/alert-dialog" attr:class=nav_link_class>{move || s().nav_alert_dialog}</A>
                    <A href="/components/drawer" attr:class=nav_link_class>{move || s().nav_drawer}</A>
                    <A href="/components/sheet" attr:class=nav_link_class>{move || s().nav_sheet}</A>
                    <A href="/components/popover" attr:class=nav_link_class>{move || s().nav_popover}</A>
                    <A href="/components/tooltip" attr:class=nav_link_class>{move || s().nav_tooltip}</A>
                    <A href="/components/hover-card" attr:class=nav_link_class>{move || s().nav_hover_card}</A>
                    <A href="/components/dropdown-menu" attr:class=nav_link_class>{move || s().nav_dropdown_menu}</A>
                    <A href="/components/context-menu" attr:class=nav_link_class>{move || s().nav_context_menu}</A>
                    <A href="/components/menubar" attr:class=nav_link_class>{move || s().nav_menubar}</A>
                    <A href="/components/command" attr:class=nav_link_class>{move || s().nav_command}</A>
                </NavSection>

                // Motion
                <NavSection title=Signal::derive(move || s().nav_motion.to_string())>
                    <A href="/components/animate" attr:class=nav_link_class>{move || s().nav_animate}</A>
                    <A href="/components/animate-group" attr:class=nav_link_class>{move || s().nav_animate_group}</A>
                    <A href="/components/marquee" attr:class=nav_link_class>{move || s().nav_marquee}</A>
                    <A href="/components/pressable" attr:class=nav_link_class>{move || s().nav_pressable}</A>
                    <A href="/components/shimmer" attr:class=nav_link_class>{move || s().nav_shimmer}</A>
                </NavSection>

                // Media
                <NavSection title=Signal::derive(move || s().nav_media.to_string())>
                    <A href="/components/carousel" attr:class=nav_link_class>{move || s().nav_carousel}</A>
                    <A href="/components/card-carousel" attr:class=nav_link_class>{move || s().nav_card_carousel}</A>
                </NavSection>

                // Interaction
                <NavSection title=Signal::derive(move || s().nav_interaction.to_string())>
                    <A href="/components/theme-toggle" attr:class=nav_link_class>{move || s().nav_theme_toggle}</A>
                    <A href="/components/direction-provider" attr:class=nav_link_class>{move || s().nav_direction_provider}</A>
                    <A href="/components/drag-and-drop" attr:class=nav_link_class>{move || s().nav_drag_and_drop}</A>
                    <A href="/components/dropzone" attr:class=nav_link_class>{move || s().nav_dropzone}</A>
                </NavSection>

                // Charts
                <NavSection title=Signal::derive(move || s().nav_charts.to_string())>
                    <A href="/charts/area" attr:class=nav_link_class>{move || s().nav_area_chart}</A>
                    <A href="/charts/bar" attr:class=nav_link_class>{move || s().nav_bar_chart}</A>
                    <A href="/charts/heatmap" attr:class=nav_link_class>{move || s().nav_heatmap}</A>
                    <A href="/charts/line" attr:class=nav_link_class>{move || s().nav_line_chart}</A>
                    <A href="/charts/pie" attr:class=nav_link_class>{move || s().nav_pie_chart}</A>
                    <A href="/charts/radar" attr:class=nav_link_class>{move || s().nav_radar_chart}</A>
                    <A href="/charts/radial" attr:class=nav_link_class>{move || s().nav_radial_chart}</A>
                    <A href="/charts/sparkline" attr:class=nav_link_class>{move || s().nav_sparkline}</A>
                    <A href="/charts/stat-tile" attr:class=nav_link_class>{move || s().nav_stat_tile}</A>
                </NavSection>

                // Blocks
                <NavSection title=Signal::derive(move || s().nav_blocks.to_string())>
                    <A href="/blocks/login" attr:class=nav_link_class>{move || s().nav_login}</A>
                    <A href="/blocks/sidenav" attr:class=nav_link_class>{move || s().nav_sidenav}</A>
                    <A href="/blocks/headers" attr:class=nav_link_class>{move || s().nav_headers}</A>
                    <A href="/blocks/footers" attr:class=nav_link_class>{move || s().nav_footers}</A>
                    <A href="/blocks/faq" attr:class=nav_link_class>{move || s().nav_faq}</A>
                    <A href="/blocks/integrations" attr:class=nav_link_class>{move || s().nav_integrations}</A>
                    <A href="/blocks/hero" attr:class=nav_link_class>{move || s().nav_hero}</A>
                    <A href="/blocks/feature-grid" attr:class=nav_link_class>{move || s().nav_feature_grid}</A>
                </NavSection>

                // Screens
                <NavSection title=Signal::derive(move || s().nav_screens.to_string())>
                    <A href="/screens/login" attr:class=nav_link_class>{move || s().nav_login_screen}</A>
                    <A href="/screens/dashboard" attr:class=nav_link_class>{move || s().nav_dashboard_screen}</A>
                    <A href="/screens/mobile" attr:class=nav_link_class>{move || s().nav_mobile_screen}</A>
                    <A href="/screens/settings" attr:class=nav_link_class>{move || s().nav_settings_screen}</A>
                </NavSection>
            </nav>

            // Mobile top bar
            <div class="flex md:hidden items-center justify-between p-4 bg-card border-b border-border w-full fixed top-0 start-0 z-40">
                <span class="font-heading font-bold text-foreground">{move || s().brand}</span>
                <button
                    class="flex items-center justify-center w-8 h-8 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
                    on:click=move |_| drawer_open.set(true)
                >
                    <span inner_html=hamburger_svg />
                </button>
            </div>

            // Mobile spacer
            <div class="h-16 md:hidden"></div>

            // Mobile drawer overlay
            <Show when=move || drawer_open.get()>
                <div class="fixed inset-0 z-50 flex">
                    // Backdrop
                    <div class="absolute inset-0 bg-black/50" on:click=move |_| drawer_open.set(false)></div>
                    // Drawer panel
                    <nav class="relative z-10 bg-card w-60 min-h-screen p-4 flex flex-col gap-6">
                        // Header
                        <div class="flex items-center gap-2">
                            <span class="font-heading text-lg font-bold text-foreground">{move || s().brand}</span>
                            <span class="bg-primary text-primary-foreground text-xs px-1.5 py-0.5 rounded font-mono">"0.1"</span>
                            <div class="ms-auto flex items-center gap-1">
                                <button
                                    class="text-xs px-2 py-1 rounded hover:bg-accent text-muted-foreground hover:text-foreground transition-colors"
                                    on:click=move |_| {
                                        locale.set(Locale::En);
                                        let _ = js_sys::eval(&format!("document.documentElement.dir='{}'", dir(Locale::En)));
                                    }
                                >
                                    {move || s().lang_en}
                                </button>
                                <button
                                    class="text-xs px-2 py-1 rounded hover:bg-accent text-muted-foreground hover:text-foreground transition-colors"
                                    on:click=move |_| {
                                        locale.set(Locale::Ar);
                                        let _ = js_sys::eval(&format!("document.documentElement.dir='{}'", dir(Locale::Ar)));
                                    }
                                >
                                    {move || s().lang_ar}
                                </button>
                                <button
                                    class="flex items-center justify-center w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
                                    on:click=on_toggle
                                >
                                    <span inner_html=move || if dark.get() { sun_svg } else { moon_svg } />
                                </button>
                            </div>
                        </div>

                        // Getting Started
                        <NavSection title=Signal::derive(move || s().nav_getting_started.to_string())>
                            <A href="/" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_home}</A>
                        </NavSection>

                        // Foundations
                        <NavSection title=Signal::derive(move || s().nav_foundations.to_string())>
                            <A href="/theme" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_theme}</A>
                        </NavSection>

                        // Hooks
                        <NavSection title=Signal::derive(move || s().nav_hooks.to_string())>
                            <A href="/hooks" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_hooks}</A>
                        </NavSection>

                        // Icons
                        <NavSection title=Signal::derive(move || s().nav_icons.to_string())>
                            <A href="/icons" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_icons}</A>
                        </NavSection>

                        // Inputs
                        <NavSection title=Signal::derive(move || s().nav_inputs.to_string())>
                            <A href="/components/button" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_button}</A>
                            <A href="/components/input" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_input}</A>
                            <A href="/components/textarea" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_textarea}</A>
                            <A href="/components/checkbox" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_checkbox}</A>
                            <A href="/components/switch" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_switch}</A>
                            <A href="/components/slider" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_slider}</A>
                            <A href="/components/radio-button" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_radio_button}</A>
                            <A href="/components/radio-button-group" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_radio_button_group}</A>
                            <A href="/components/toggle-group" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_toggle_group}</A>
                            <A href="/components/button-group" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_button_group}</A>
                            <A href="/components/button-action" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_button_action}</A>
                            <A href="/components/input-group" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_input_group}</A>
                            <A href="/components/select" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_select}</A>
                            <A href="/components/combobox" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_combobox}</A>
                            <A href="/components/multi-select" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_multi_select}</A>
                            <A href="/components/date-picker" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_date_picker}</A>
                            <A href="/components/input-otp" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_input_otp}</A>
                            <A href="/components/input-phone" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_input_phone}</A>
                            <A href="/components/input-prompt" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_input_prompt}</A>
                        </NavSection>

                        // Forms
                        <NavSection title=Signal::derive(move || s().nav_forms.to_string())>
                            <A href="/components/label" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_label}</A>
                            <A href="/components/field" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_field}</A>
                            <A href="/components/form" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_form}</A>
                            <A href="/components/auto-form" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_auto_form}</A>
                        </NavSection>

                        // Data Display
                        <NavSection title=Signal::derive(move || s().nav_data_display.to_string())>
                            <A href="/components/badge" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_badge}</A>
                            <A href="/components/card" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_card}</A>
                            <A href="/components/stat" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_stat}</A>
                            <A href="/components/avatar" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_avatar}</A>
                            <A href="/components/kbd" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_kbd}</A>
                            <A href="/components/chip" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_chip}</A>
                            <A href="/components/empty" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_empty}</A>
                            <A href="/components/item" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_item}</A>
                            <A href="/components/table" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_table}</A>
                            <A href="/components/data-table" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_data_table}</A>
                            <A href="/components/data-grid" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_data_grid}</A>
                            <A href="/components/code-block" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_code_block}</A>
                            <A href="/components/timeline" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_timeline}</A>
                            <A href="/components/comparison-table" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_comparison_table}</A>
                        </NavSection>

                        // Feedback
                        <NavSection title=Signal::derive(move || s().nav_feedback.to_string())>
                            <A href="/components/skeleton" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_skeleton}</A>
                            <A href="/components/spinner" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_spinner}</A>
                            <A href="/components/alert" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_alert}</A>
                            <A href="/components/callout" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_callout}</A>
                            <A href="/components/progress" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_progress}</A>
                            <A href="/components/status" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_status}</A>
                            <A href="/components/toast" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_toast}</A>
                            <A href="/components/sonner" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_sonner}</A>
                        </NavSection>

                        // Layout
                        <NavSection title=Signal::derive(move || s().nav_layout.to_string())>
                            <A href="/components/separator" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_separator}</A>
                            <A href="/components/aspect-ratio" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_aspect_ratio}</A>
                            <A href="/components/scroll-area" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_scroll_area}</A>
                            <A href="/components/page-header" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_page_header}</A>
                        </NavSection>

                        // Navigation
                        <NavSection title=Signal::derive(move || s().nav_navigation.to_string())>
                            <A href="/components/breadcrumb" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_breadcrumb}</A>
                            <A href="/components/tabs" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_tabs}</A>
                            <A href="/components/pagination" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_pagination}</A>
                            <A href="/components/bottom-nav" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_bottom_nav}</A>
                            <A href="/components/navigation-menu" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_navigation_menu}</A>
                            <A href="/components/sidebar" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_sidebar}</A>
                        </NavSection>

                        // Disclosure
                        <NavSection title=Signal::derive(move || s().nav_disclosure.to_string())>
                            <A href="/components/accordion" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_accordion}</A>
                            <A href="/components/collapsible" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_collapsible}</A>
                        </NavSection>

                        // Overlays
                        <NavSection title=Signal::derive(move || s().nav_overlays.to_string())>
                            <A href="/components/dialog" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_dialog}</A>
                            <A href="/components/alert-dialog" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_alert_dialog}</A>
                            <A href="/components/drawer" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_drawer}</A>
                            <A href="/components/sheet" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_sheet}</A>
                            <A href="/components/popover" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_popover}</A>
                            <A href="/components/tooltip" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_tooltip}</A>
                            <A href="/components/hover-card" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_hover_card}</A>
                            <A href="/components/dropdown-menu" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_dropdown_menu}</A>
                            <A href="/components/context-menu" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_context_menu}</A>
                            <A href="/components/menubar" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_menubar}</A>
                            <A href="/components/command" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_command}</A>
                        </NavSection>

                        // Motion
                        <NavSection title=Signal::derive(move || s().nav_motion.to_string())>
                            <A href="/components/animate" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_animate}</A>
                            <A href="/components/animate-group" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_animate_group}</A>
                            <A href="/components/marquee" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_marquee}</A>
                            <A href="/components/pressable" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_pressable}</A>
                            <A href="/components/shimmer" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_shimmer}</A>
                        </NavSection>

                        // Media
                        <NavSection title=Signal::derive(move || s().nav_media.to_string())>
                            <A href="/components/carousel" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_carousel}</A>
                            <A href="/components/card-carousel" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_card_carousel}</A>
                        </NavSection>

                        // Interaction
                        <NavSection title=Signal::derive(move || s().nav_interaction.to_string())>
                            <A href="/components/theme-toggle" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_theme_toggle}</A>
                            <A href="/components/direction-provider" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_direction_provider}</A>
                            <A href="/components/drag-and-drop" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_drag_and_drop}</A>
                            <A href="/components/dropzone" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_dropzone}</A>
                        </NavSection>

                        // Charts
                        <NavSection title=Signal::derive(move || s().nav_charts.to_string())>
                            <A href="/charts/area" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_area_chart}</A>
                            <A href="/charts/bar" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_bar_chart}</A>
                            <A href="/charts/heatmap" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_heatmap}</A>
                            <A href="/charts/line" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_line_chart}</A>
                            <A href="/charts/pie" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_pie_chart}</A>
                            <A href="/charts/radar" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_radar_chart}</A>
                            <A href="/charts/radial" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_radial_chart}</A>
                            <A href="/charts/sparkline" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_sparkline}</A>
                            <A href="/charts/stat-tile" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_stat_tile}</A>
                        </NavSection>

                        // Blocks
                        <NavSection title=Signal::derive(move || s().nav_blocks.to_string())>
                            <A href="/blocks/login" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_login}</A>
                            <A href="/blocks/sidenav" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_sidenav}</A>
                            <A href="/blocks/headers" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_headers}</A>
                            <A href="/blocks/footers" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_footers}</A>
                            <A href="/blocks/faq" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_faq}</A>
                            <A href="/blocks/integrations" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_integrations}</A>
                            <A href="/blocks/hero" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_hero}</A>
                            <A href="/blocks/feature-grid" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_feature_grid}</A>
                        </NavSection>

                        // Screens
                        <NavSection title=Signal::derive(move || s().nav_screens.to_string())>
                            <A href="/screens/login" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_login_screen}</A>
                            <A href="/screens/dashboard" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_dashboard_screen}</A>
                            <A href="/screens/mobile" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_mobile_screen}</A>
                            <A href="/screens/settings" attr:class=nav_link_class on:click=move |_| drawer_open.set(false)>{move || s().nav_settings_screen}</A>
                        </NavSection>
                    </nav>
                </div>
            </Show>
        </>
    }
}
