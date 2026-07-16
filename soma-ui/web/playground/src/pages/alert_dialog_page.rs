use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, Button,
};

#[component]
pub fn AlertDialogPage() -> impl IntoView {
    let open = RwSignal::new(false);
    let confirmed = RwSignal::new(false);

    view! {
        <PageShell
            title=Signal::derive(move || "Alert Dialog".to_string())
            subtitle=Signal::derive(move || "A confirmation dialog that requires explicit user action. Backdrop clicks do not dismiss it.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-52">
                <Button on:click=move |_| open.set(true)>
                    "Delete Account"
                </Button>
                <Show when=move || confirmed.get()>
                    <p class="text-sm text-destructive">"Action confirmed."</p>
                </Show>
                <AlertDialog open=open>
                    <AlertDialogContent>
                        <AlertDialogHeader>
                            <AlertDialogTitle>"Delete Account"</AlertDialogTitle>
                            <AlertDialogDescription>"This will permanently delete your account and all associated data. This action cannot be undone."</AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                            <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                            <AlertDialogAction on_click=Callback::new(move |_| confirmed.set(true))>
                                "Yes, delete"
                            </AlertDialogAction>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialog>
            </div>
        </PageShell>
    }
}
