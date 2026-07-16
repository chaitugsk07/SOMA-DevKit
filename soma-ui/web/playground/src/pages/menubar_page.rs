use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarTrigger,
};

#[component]
pub fn MenubarPage() -> impl IntoView {
    let show_view_menu = RwSignal::new(true);
    let show_separator = RwSignal::new(true);
    let last_clicked = RwSignal::new("None".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Menubar".to_string())
            subtitle=Signal::derive(move || "A horizontal menu bar with multiple dropdown menus. Hover switches between open menus.".to_string())
        >
            <div class="bg-card border border-border rounded-md p-6 md:p-12 flex flex-col items-center justify-center gap-4 min-h-72">
                {move || {
                    let sep = show_separator.get();
                    let view_menu = show_view_menu.get();
                    view! {
                        <Menubar>
                            <MenubarMenu value="file">
                                <MenubarTrigger>"File"</MenubarTrigger>
                                <MenubarContent>
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("New".to_string()))>"New"</MenubarItem>
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Open".to_string()))>"Open"</MenubarItem>
                                    {sep.then(|| view! { <MenubarSeparator /> })}
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Save".to_string()))>"Save"</MenubarItem>
                                </MenubarContent>
                            </MenubarMenu>
                            <MenubarMenu value="edit">
                                <MenubarTrigger>"Edit"</MenubarTrigger>
                                <MenubarContent>
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Undo".to_string()))>"Undo"</MenubarItem>
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Redo".to_string()))>"Redo"</MenubarItem>
                                    {sep.then(|| view! { <MenubarSeparator /> })}
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Cut".to_string()))>"Cut"</MenubarItem>
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Copy".to_string()))>"Copy"</MenubarItem>
                                    <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Paste".to_string()))>"Paste"</MenubarItem>
                                </MenubarContent>
                            </MenubarMenu>
                            {view_menu.then(|| view! {
                                <MenubarMenu value="view">
                                    <MenubarTrigger>"View"</MenubarTrigger>
                                    <MenubarContent>
                                        <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Zoom In".to_string()))>"Zoom In"</MenubarItem>
                                        <MenubarItem on_click=Callback::new(move |_| last_clicked.set("Zoom Out".to_string()))>"Zoom Out"</MenubarItem>
                                    </MenubarContent>
                                </MenubarMenu>
                            })}
                        </Menubar>
                    }
                }}
                <p class="text-xs text-muted-foreground">"Last selected: " {move || last_clicked.get()}</p>
            </div>

            <ControlsPanel>
                <ControlRow label="Show View menu">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| show_view_menu.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Show separators">
                    <input
                        type="checkbox"
                        checked
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        on:change=move |e| show_separator.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
