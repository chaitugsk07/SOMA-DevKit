use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn Separator(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let (orientation_class, role_attr) = match orientation {
        Orientation::Horizontal => ("h-px w-full bg-border", "separator"),
        Orientation::Vertical => ("w-px h-full bg-border", "separator"),
    };

    let combined = format!("{} {}", orientation_class, class);

    view! {
        <div
            role=role_attr
            aria-orientation=match orientation {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
            class=combined
        />
    }
}
