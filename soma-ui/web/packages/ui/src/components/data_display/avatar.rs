use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Avatar(
    #[prop(optional)] src: Option<String>,
    #[prop(default = String::new())] alt: String,
    #[prop(default = String::new())] fallback: String,
    #[prop(default = AvatarSize::Md)] size: AvatarSize,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let size_class = match size {
        AvatarSize::Sm => "h-8 w-8 text-xs",
        AvatarSize::Md => "h-10 w-10 text-sm",
        AvatarSize::Lg => "h-12 w-12 text-base",
    };
    let combined = format!(
        "rounded-full overflow-hidden bg-muted flex items-center justify-center shrink-0 shadow-elev-sm {} {}",
        size_class, class
    );
    view! {
        <div class=combined>
            {if let Some(s) = src {
                view! { <img src=s alt=alt class="h-full w-full object-cover" /> }.into_any()
            } else {
                view! { <span class="font-medium text-muted-foreground">{fallback}</span> }.into_any()
            }}
        </div>
    }
}
