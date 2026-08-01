use leptos::prelude::*;

/// Top navigation bar with soma_ logo (blinking cursor in CSS) and nav links.
/// Pass the `ThemeToggle` island (or any interactive widget) as `children`.
/// Children are always rendered; caller decides whether to include any.
///
/// `nav_links` tuples: `(href, label, is_secondary)`.
/// Secondary links receive `.nav-link--secondary` and are hidden at ≤480 px.
///
/// `badge` — optional text to display as a small uppercase chip next to the
/// logo (e.g. `badge="BETA"`). Omit to render no chip.
#[component]
pub fn SiteHeader(
    /// Current path for active-link highlighting (e.g. "/blog")
    #[prop(optional, into)]
    current_path: String,
    /// Optional badge label rendered next to the logo (e.g. "BETA").
    #[prop(optional, into)]
    badge: String,
    /// Nav links as (href, label, is_secondary) triples.
    nav_links: Vec<(&'static str, String, bool)>,
    /// Slot for the ThemeToggle island (or any other interactive widget)
    children: Children,
) -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="container site-header__inner">
                <a href="/" class="site-header__logo" aria-label="SOMA home">
                    <span class="logo-word">"soma"</span>
                    <span class="logo-cursor" aria-hidden="true">"_"</span>
                </a>
                {(!badge.is_empty()).then(|| view! {
                    <span class="logo-beta" aria-label="beta release">{badge}</span>
                })}
                <details class="nav-disclosure">
                    <summary class="nav-disclosure__toggle" aria-label="Navigation menu">"☰"</summary>
                    <nav class="site-header__nav" aria-label="Main navigation">
                        {nav_links.into_iter().map(|(href, label, is_secondary)| {
                            let is_active = current_path.starts_with(href);
                            let cls = match (is_active, is_secondary) {
                                (true,  false) => "nav-link nav-link--active",
                                (true,  true)  => "nav-link nav-link--active nav-link--secondary",
                                (false, true)  => "nav-link nav-link--secondary",
                                (false, false) => "nav-link",
                            };
                            view! {
                                <a href=href class=cls>{label}</a>
                            }
                        }).collect_view()}
                    </nav>
                </details>
                <div class="site-header__actions">
                    {children()}
                </div>
            </div>
        </header>
    }
}
