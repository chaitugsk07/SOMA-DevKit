use leptos::prelude::*;

/// Status of a suite or feature. Controls the LED color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BadgeStatus {
    /// Production-ready and deployed — green LED
    Shipped,
    /// Working but not yet hardened — amber LED
    Usable,
    /// In active development — warn LED
    Wip,
    /// Planned, not yet started — muted LED
    Stub,
}

impl BadgeStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Shipped => "shipped",
            Self::Usable  => "usable",
            Self::Wip     => "wip",
            Self::Stub    => "stub",
        }
    }

    fn css_mod(self) -> &'static str {
        match self {
            Self::Shipped => "status-badge--shipped",
            Self::Usable  => "status-badge--usable",
            Self::Wip     => "status-badge--wip",
            Self::Stub    => "status-badge--stub",
        }
    }
}

/// Square LED glyph ■ + status label. Colors: shipped=ok, usable=amber,
/// wip=warn, stub=muted.
#[component]
pub fn StatusBadge(status: BadgeStatus) -> impl IntoView {
    let cls = format!("status-badge {}", status.css_mod());
    view! {
        <span class=cls>
            <span class="status-badge__led" aria-hidden="true">"■ "</span>
            {status.label()}
        </span>
    }
}
