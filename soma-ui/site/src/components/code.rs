use leptos::prelude::*;

/// Renders pre-highlighted HTML from the syntect build pipeline.
/// `highlighted_html` is the raw HTML string with inline `<span>` elements.
/// The copy-button slot is optional interactive content (use an island for it).
#[component]
pub fn CodeBlock(
    /// Pre-highlighted HTML string from `comrak` + `syntect`
    #[prop(into)]
    highlighted_html: String,
    /// Language label shown in the top-right corner (e.g. "toml", "rust")
    #[prop(optional, into)]
    lang: String,
    /// Optional slot: pass a copy-button island here
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let show_header = !lang.is_empty() || children.is_some();

    view! {
        <div class="code-block">
            {show_header.then(|| {
                let lang_clone = lang.clone();
                view! {
                    <div class="code-block__header">
                        {if !lang_clone.is_empty() {
                            Some(view! {
                                <span class="code-block__lang">{lang_clone}</span>
                            })
                        } else {
                            None
                        }}
                        {children.map(|c| c())}
                    </div>
                }
            })}
            <div class="code-block__body" inner_html=highlighted_html></div>
        </div>
    }
}
