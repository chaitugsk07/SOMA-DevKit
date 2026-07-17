use leptos::prelude::*;

/// A horizontal step progress rail with numbered cells and an active highlight.
#[component]
pub fn StepRail(
    steps: Vec<String>,
    active: usize,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let n = steps.len();
    let outer = format!("grid overflow-hidden rounded-md border border-border {}", class);
    let cols = format!("grid-template-columns: repeat({}, 1fr);", n);

    view! {
        <div class=outer style=cols>
            {steps.into_iter().enumerate().map(|(i, step)| {
                let cell_class = if i == active {
                    "flex flex-col gap-0.5 px-3 py-2 text-center border-r border-border last:border-r-0 \
                     bg-[color:var(--soma-brand-highlight,hsl(var(--accent)))] text-foreground"
                } else {
                    "flex flex-col gap-0.5 px-3 py-2 text-center border-r border-border last:border-r-0"
                };
                view! {
                    <div class=cell_class>
                        <span class="text-xs font-bold">{i + 1}</span>
                        <span class="text-[10px] uppercase tracking-wider text-muted-foreground">{step}</span>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
