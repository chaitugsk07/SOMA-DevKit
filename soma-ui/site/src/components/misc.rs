use leptos::prelude::*;

/// Inline tag / label chip (e.g. a category or technology label).
#[component]
pub fn Tag(children: Children) -> impl IntoView {
    view! { <span class="tag">{children()}</span> }
}

/// Keyboard key indicator — renders as `<kbd>`.
#[component]
pub fn KbdKey(children: Children) -> impl IntoView {
    view! { <kbd class="kbd-key">{children()}</kbd> }
}

/// Horizontal rule made from ASCII/box-drawing characters.
/// If `label` is provided it floats centred in the rule.
#[component]
pub fn AsciiDivider(
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    if label.is_empty() {
        view! {
            <div class="ascii-divider" aria-hidden="true">
                <span class="ascii-divider__rule">"╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌"</span>
            </div>
        }.into_any()
    } else {
        view! {
            <div class="ascii-divider ascii-divider--labeled" role="separator">
                <span class="ascii-divider__rule" aria-hidden="true">"╌╌╌╌╌╌╌╌╌╌╌╌"</span>
                <span class="ascii-divider__label">{label}</span>
                <span class="ascii-divider__rule" aria-hidden="true">"╌╌╌╌╌╌╌╌╌╌╌╌"</span>
            </div>
        }.into_any()
    }
}

/// Typographic wrapper for rendered markdown HTML.
/// Sets headings, links (amber underline offset), tables, blockquotes
/// (rust left-rule), and syntect code blocks to palette.
/// Pass `highlighted_html` from the content pipeline.
#[component]
pub fn ProseArticle(
    #[prop(into)] html: String,
) -> impl IntoView {
    view! {
        <article class="prose" inner_html=html></article>
    }
}
