use leptos::prelude::*;

/// A column of links in the site footer.
pub struct FooterCol {
    pub title: String,
    pub links: Vec<(&'static str, String)>,
}

/// Site footer with brand, tagline, link columns, and copyright.
/// Callers supply locale-translated strings; product names and anchor hrefs stay in-component.
#[component]
pub fn SiteFooter(
    #[prop(into)] tagline: String,
    /// Footer link columns; each has a heading and (href, label) link pairs.
    cols: Vec<FooterCol>,
    #[prop(into)] copyright: String,
) -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="container site-footer__inner">
                <div class="site-footer__brand">
                    <span class="logo-word">"soma"</span>
                    <span class="logo-cursor" aria-hidden="true">"_"</span>
                    <p class="site-footer__tagline">{tagline}</p>
                </div>

                <nav class="site-footer__links" aria-label="Footer navigation">
                    {cols.into_iter().map(|col| {
                        view! {
                            <div class="site-footer__col">
                                <p class="site-footer__col-title">{col.title}</p>
                                {col.links.into_iter().map(|(href, label)| {
                                    view! { <a href=href>{label}</a> }
                                }).collect_view()}
                            </div>
                        }
                    }).collect_view()}
                </nav>
            </div>

            <div class="site-footer__bottom">
                <div class="container">
                    <span class="site-footer__copy">{copyright}</span>
                    <span class="site-footer__ascii" aria-hidden="true">
                        "╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌"
                    </span>
                </div>
            </div>
        </footer>
    }
}
