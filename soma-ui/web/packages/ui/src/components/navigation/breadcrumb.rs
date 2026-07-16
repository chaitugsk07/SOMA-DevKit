use crate::components::shared::CONTROL_MOTION;
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

#[component]
pub fn Breadcrumb(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground {}",
        class
    );
    view! {
        <nav aria-label="breadcrumb">
            <ol class=combined>{children()}</ol>
        </nav>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("inline-flex items-center gap-1.5 {}", class);
    view! {
        <li class=combined>{children()}</li>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(default = String::new())] href: String,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("hover:text-foreground {} {}", CONTROL_MOTION, class);
    view! {
        <a href=href class=combined>{children()}</a>
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("text-foreground font-normal {}", class);
    view! {
        <span aria-current="page" class=combined>{children()}</span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <li role="presentation" aria-hidden="true" class=class>
            {match children {
                Some(c) => view! { <>{c()}</> }.into_any(),
                None => view! {
                    <Icon icon=Signal::derive(|| icondata::LuChevronRight) width="16" height="16"/>
                }.into_any(),
            }}
        </li>
    }
}
