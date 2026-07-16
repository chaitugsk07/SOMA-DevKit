use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    Button, ButtonVariant, Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader,
    SheetSide, SheetTitle,
};

#[component]
pub fn SheetPage() -> impl IntoView {
    let open = RwSignal::new(false);
    let side = RwSignal::new(SheetSide::Right);

    view! {
        <PageShell
            title=Signal::derive(|| "Sheet".to_string())
            subtitle=Signal::derive(|| "A slide-over panel from any edge. Closes on backdrop click or Escape.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-52">
                <div class="flex flex-wrap gap-3 justify-center">
                    <Button on:click=move |_| { side.set(SheetSide::Right); open.set(true); }>
                        "Right (default)"
                    </Button>
                    <Button variant=ButtonVariant::Outline on:click=move |_| { side.set(SheetSide::Left); open.set(true); }>
                        "Left"
                    </Button>
                    <Button variant=ButtonVariant::Outline on:click=move |_| { side.set(SheetSide::Top); open.set(true); }>
                        "Top"
                    </Button>
                    <Button variant=ButtonVariant::Outline on:click=move |_| { side.set(SheetSide::Bottom); open.set(true); }>
                        "Bottom"
                    </Button>
                </div>
                // ponytail: reconstruct Sheet each time open is toggled so side prop is fresh.
                <Show when=move || open.get()>
                    {move || {
                        let current_side = side.get();
                        view! {
                            <Sheet open=open side=current_side>
                                <SheetContent>
                                    <SheetHeader>
                                        <SheetTitle>"Side Panel"</SheetTitle>
                                        <SheetDescription>"This sheet slides in from the selected side."</SheetDescription>
                                    </SheetHeader>
                                    <div class="py-4 space-y-2">
                                        <p class="text-sm text-muted-foreground">"Content can go here — forms, navigation, filters, or any other UI."</p>
                                    </div>
                                    <SheetFooter>
                                        <Button variant=ButtonVariant::Outline on:click=move |_| open.set(false)>
                                            "Close"
                                        </Button>
                                    </SheetFooter>
                                </SheetContent>
                            </Sheet>
                        }
                    }}
                </Show>
            </div>
        </PageShell>
    }
}
