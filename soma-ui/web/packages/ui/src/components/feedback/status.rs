use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum StatusKind {
    #[default]
    Online,
    Offline,
    Away,
    Busy,
}

#[component]
pub fn Status(
    #[prop(default = StatusKind::Online)] kind: StatusKind,
    #[prop(optional)] label: Option<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let dot_class = match kind {
        StatusKind::Online => "bg-success",
        StatusKind::Offline => "bg-muted-foreground",
        StatusKind::Away => "bg-warning",
        StatusKind::Busy => "bg-destructive",
    };
    let combined = format!("inline-flex items-center gap-2 {}", class);
    view! {
        <span class=combined>
            <span class=format!("h-2 w-2 rounded-full {}", dot_class)></span>
            {label.map(|l| view! { <span class="text-sm text-muted-foreground">{l}</span> })}
        </span>
    }
}
