use crate::components::shared::{CONTROL_MOTION, FOCUS_RING_INSET};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    #[default]
    Segment,
    Pill,
    Underline,
}

#[component]
pub fn Tabs(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(value);
    let combined = format!("w-full {}", class);
    view! {
        <div class=combined>{children()}</div>
    }
}

#[component]
pub fn TabsList(
    #[prop(default = TabsVariant::Segment)] variant: TabsVariant,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(variant);
    let combined = match variant {
        TabsVariant::Segment => format!(
            "inline-flex h-9 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground {}",
            class
        ),
        TabsVariant::Pill => format!("inline-flex items-center gap-2 text-muted-foreground {}", class),
        TabsVariant::Underline => format!(
            "inline-flex items-center border-b border-border text-muted-foreground {}",
            class
        ),
    };
    view! {
        <div role="tablist" class=combined>{children()}</div>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let active = use_context::<RwSignal<String>>().expect("TabsTrigger must be used inside Tabs");
    let variant = use_context::<TabsVariant>().unwrap_or_default();
    let val = StoredValue::new(value);
    view! {
        <button
            role="tab"
            class=move || {
                let is_active = active.get() == val.get_value();
                let extra = if class.is_empty() { "" } else { &class };
                match variant {
                    TabsVariant::Segment => {
                        let base = format!(
                            "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1 text-sm font-medium {} {}",
                            FOCUS_RING_INSET, CONTROL_MOTION
                        );
                        if is_active {
                            format!("{} bg-background text-foreground shadow-elev-sm {}", base, extra)
                        } else {
                            format!("{} hover:bg-muted/50 hover:text-foreground {}", base, extra)
                        }
                    }
                    TabsVariant::Pill => {
                        if is_active {
                            format!(
                                "inline-flex items-center justify-center whitespace-nowrap rounded-full px-4 py-1.5 text-sm font-medium bg-contrast text-contrast-foreground {} {} {}",
                                FOCUS_RING_INSET, CONTROL_MOTION, extra
                            )
                        } else {
                            format!(
                                "inline-flex items-center justify-center whitespace-nowrap rounded-full px-4 py-1.5 text-sm font-medium border border-input bg-card text-muted-foreground hover:bg-muted {} {} {}",
                                FOCUS_RING_INSET, CONTROL_MOTION, extra
                            )
                        }
                    }
                    TabsVariant::Underline => {
                        if is_active {
                            format!(
                                "inline-flex items-center justify-center whitespace-nowrap px-4 py-2 text-sm font-medium border-b-2 border-primary text-foreground {} {} {}",
                                FOCUS_RING_INSET, CONTROL_MOTION, extra
                            )
                        } else {
                            format!(
                                "inline-flex items-center justify-center whitespace-nowrap px-4 py-2 text-sm font-medium border-b-2 border-transparent text-muted-foreground hover:text-foreground {} {} {}",
                                FOCUS_RING_INSET, CONTROL_MOTION, extra
                            )
                        }
                    }
                }
            }
            aria-selected=move || if active.get() == val.get_value() { "true" } else { "false" }
            on:click=move |_| active.set(val.get_value())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    #[prop(default = String::new())] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let active = use_context::<RwSignal<String>>().expect("TabsContent must be used inside Tabs");
    let val = StoredValue::new(value);
    let combined = format!("mt-2 focus:outline-none {}", class);
    view! {
        <Show when=move || active.get() == val.get_value()>
            <div role="tabpanel" class=combined.clone()>
                {children()}
            </div>
        </Show>
    }
}
