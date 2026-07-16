use crate::ui::*;
use leptos::prelude::*;
use soma_ui::ThemeToggle;

#[component]
pub fn ThemeTogglePage() -> impl IntoView {
    // ThemeToggle only accepts an optional `class` prop — no size/variant variants exist.
    // The control here lets the user add extra Tailwind classes to see layout effects.
    let extra_class = RwSignal::new("".to_string());

    view! {
        <PageShell
            title=Signal::derive(move || "Theme Toggle".to_string())
            subtitle=Signal::derive(move || "A self-contained button that toggles the global dark/light theme by adding or removing the 'dark' class on the root element.".to_string())
        >
            <PreviewPanel>
                {move || view! {
                    <ThemeToggle class=extra_class.get() />
                }}
            </PreviewPanel>

            <ControlsPanel>
                <ControlRow label="Size override">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            extra_class.set(match event_target_value(&e).as_str() {
                                "sm" => "!w-7 !h-7".to_string(),
                                "lg" => "!w-12 !h-12".to_string(),
                                _ => "".to_string(),
                            });
                        }
                    >
                        <option value="default" selected>"Default (36px)"</option>
                        <option value="sm">"Small (28px)"</option>
                        <option value="lg">"Large (48px)"</option>
                    </select>
                </ControlRow>
            </ControlsPanel>

            <div class="bg-card border border-border rounded-md p-6">
                <h2 class="text-sm font-semibold text-foreground mb-2">"Notes"</h2>
                <ul class="text-sm text-muted-foreground space-y-1 list-disc list-inside">
                    <li>"Reads initial state from documentElement.classList on mount."</li>
                    <li>"Persists preference to localStorage under the key 'theme'."</li>
                    <li>"Shows a sun icon in dark mode, moon icon in light mode."</li>
                    <li>"The sidebar also contains a theme toggle — both operate on the same document class."</li>
                    <li>"ThemeToggle has no size/variant props; use the class prop to override Tailwind sizing."</li>
                </ul>
            </div>
        </PageShell>
    }
}
