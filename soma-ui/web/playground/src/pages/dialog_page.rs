use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    Button, ButtonVariant, Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle,
};

#[component]
pub fn DialogPage() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(move || "Dialog".to_string())
            subtitle=Signal::derive(move || "A modal dialog that renders in a portal above the page. Closes on backdrop click or Escape.".to_string())
        >
            <PreviewPanel>
                <Button on:click=move |_| open.set(true)>
                    "Open Dialog"
                </Button>
                <Dialog open=open>
                    <DialogContent>
                        <DialogHeader>
                            <DialogTitle>"Confirm Action"</DialogTitle>
                            <DialogDescription>"Are you sure you want to proceed? This action cannot be undone."</DialogDescription>
                        </DialogHeader>
                        <DialogFooter>
                            <Button variant=ButtonVariant::Outline on:click=move |_| open.set(false)>
                                "Cancel"
                            </Button>
                            <Button on:click=move |_| open.set(false)>
                                "Confirm"
                            </Button>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            </PreviewPanel>
        </PageShell>
    }
}
