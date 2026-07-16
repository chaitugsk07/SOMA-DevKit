use leptos::prelude::*;

#[component]
pub fn Hero(
    #[prop(into, optional)] eyebrow: Option<String>,
    #[prop(into)] title: String,
    #[prop(into, optional)] subtitle: Option<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let combined = format!(
        "flex flex-col items-center justify-center text-center px-4 py-24 {}",
        class
    );

    let eyebrow_view = eyebrow.map(|e| {
        view! {
            <p class="text-xs font-semibold uppercase tracking-widest text-muted-foreground">{e}</p>
        }
    });
    let subtitle_view = subtitle.map(|s| {
        view! {
            <p class="text-lg text-muted-foreground max-w-xl mx-auto">{s}</p>
        }
    });

    view! {
        <section class=combined>
            <div class="animate-fade-in max-w-3xl mx-auto space-y-6">
                {eyebrow_view}
                <h1 class="font-heading text-4xl sm:text-5xl font-bold tracking-tight text-foreground">{title}</h1>
                {subtitle_view}
                <div class="flex flex-wrap items-center justify-center gap-3">
                    {children()}
                </div>
            </div>
        </section>
    }
}
