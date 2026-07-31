use leptos::prelude::*;

/// Note/warning callout block.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CalloutKind {
    Note,
    Warn,
}

#[component]
pub fn Callout(kind: CalloutKind, children: Children) -> impl IntoView {
    let (cls, label) = match kind {
        CalloutKind::Note => ("callout callout--note", "Note"),
        CalloutKind::Warn => ("callout callout--warn", "Warning"),
    };
    view! {
        <aside class=cls role="note">
            <span class="callout__label">{label}</span>
            <div class="callout__body">{children()}</div>
        </aside>
    }
}
