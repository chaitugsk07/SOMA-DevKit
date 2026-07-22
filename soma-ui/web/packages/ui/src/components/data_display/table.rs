use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TableDensity {
    #[default]
    Comfortable,
    Compact,
}

// Private marker for sticky-header context — avoids raw `bool` context collisions.
#[derive(Clone, Copy)]
struct StickyHeader(bool);

#[component]
pub fn Table(
    #[prop(default = TableDensity::Comfortable)] density: TableDensity,
    #[prop(default = false)] sticky_header: bool,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(density);
    provide_context(StickyHeader(sticky_header));
    let wrapper = format!(
        "rounded-xl border border-border bg-card overflow-x-auto {}",
        class
    );
    view! {
        <div class=wrapper>
            <table class="w-full caption-bottom text-sm">{children()}</table>
        </div>
    }
}

#[component]
pub fn TableHeader(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let sticky = use_context::<StickyHeader>().map(|s| s.0).unwrap_or(false);
    let sticky_cls = if sticky { " sticky top-0 z-10" } else { "" };
    let combined = format!("[&_tr]:border-b bg-muted{} {}", sticky_cls, class);
    view! { <thead class=combined>{children()}</thead> }
}

#[component]
pub fn TableBody(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("[&_tr:last-child]:border-0 {}", class);
    view! { <tbody class=combined>{children()}</tbody> }
}

#[component]
pub fn TableFooter(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("border-t border-border bg-muted/50 font-medium {}", class);
    view! { <tfoot class=combined>{children()}</tfoot> }
}

#[component]
pub fn TableRow(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "border-b border-border/60 transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted {}",
        class
    );
    view! { <tr class=combined>{children()}</tr> }
}

#[component]
pub fn TableHead(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let density = use_context::<TableDensity>().unwrap_or_default();
    let padding = match density {
        TableDensity::Comfortable => "h-10 px-3",
        TableDensity::Compact => "h-8 px-2",
    };
    let combined = format!(
        "{} text-start align-middle font-medium text-muted-foreground {}",
        padding, class
    );
    view! { <th class=combined>{children()}</th> }
}

#[component]
pub fn TableCell(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let density = use_context::<TableDensity>().unwrap_or_default();
    let padding = match density {
        TableDensity::Comfortable => "p-3",
        TableDensity::Compact => "p-2 text-xs",
    };
    let combined = format!("{} align-middle text-foreground {}", padding, class);
    view! { <td class=combined>{children()}</td> }
}

#[component]
pub fn TableCaption(
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!("mt-4 text-sm text-muted-foreground {}", class);
    view! { <caption class=combined>{children()}</caption> }
}
