use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    Button, ButtonVariant, Drawer, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader,
    DrawerTitle,
};

#[component]
pub fn DrawerPage() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(move || "Drawer".to_string())
            subtitle=Signal::derive(move || "A panel that slides up from the bottom of the screen. Closes on backdrop click or Escape.".to_string())
        >
            <PreviewPanel>
                <Button on:click=move |_| open.set(true)>
                    "Open Drawer"
                </Button>
                <Drawer open=open>
                    <DrawerContent>
                        <DrawerHeader>
                            <DrawerTitle>"Quick Settings"</DrawerTitle>
                            <DrawerDescription>"Adjust your preferences below. Changes are saved automatically."</DrawerDescription>
                        </DrawerHeader>
                        <div class="space-y-3 py-4">
                            <p class="text-sm text-muted-foreground">"Notifications — enabled"</p>
                            <p class="text-sm text-muted-foreground">"Dark mode — on"</p>
                            <p class="text-sm text-muted-foreground">"Language — English"</p>
                        </div>
                        <DrawerFooter>
                            <Button variant=ButtonVariant::Outline on:click=move |_| open.set(false)>
                                "Close"
                            </Button>
                        </DrawerFooter>
                    </DrawerContent>
                </Drawer>
            </PreviewPanel>
        </PageShell>
    }
}
