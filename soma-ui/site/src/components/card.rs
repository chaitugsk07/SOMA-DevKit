use leptos::prelude::*;
use super::badge::{BadgeStatus, StatusBadge};

/// A suite card: name, status badge, short description, link.
/// Blueprint corner ticks are applied via CSS ::before/::after.
#[component]
pub fn SuiteCard(
    #[prop(into)] name: String,
    #[prop(into)] description: String,
    #[prop(into)] href: String,
    status: BadgeStatus,
) -> impl IntoView {
    view! {
        <article class="suite-card">
            <div class="suite-card__head">
                <h3 class="suite-card__name">{name}</h3>
                <StatusBadge status=status/>
            </div>
            <p class="suite-card__desc">{description}</p>
            <a href=href class="suite-card__link">"Learn more →"</a>
        </article>
    }
}

/// A feature highlight card: icon/title area + description body via children.
#[component]
pub fn FeatureCard(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <article class="feature-card">
            <h3 class="feature-card__title">{title}</h3>
            <div class="feature-card__body">{children()}</div>
        </article>
    }
}
