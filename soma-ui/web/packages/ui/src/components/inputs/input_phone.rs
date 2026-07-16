use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use crate::{Select, SelectContent, SelectItem};
use leptos::prelude::*;

// ponytail: ~12 common country codes; auto-format strips non-digits only.
// Ceiling: no libphonenumber formatting, no flag icons, no search.
// Upgrade path: add phonenumber crate + flag emoji map, or country-search combobox.

const COUNTRIES: &[(&str, &str)] = &[
    ("+1", "US +1"),
    ("+44", "UK +44"),
    ("+91", "IN +91"),
    ("+61", "AU +61"),
    ("+49", "DE +49"),
    ("+33", "FR +33"),
    ("+81", "JP +81"),
    ("+86", "CN +86"),
    ("+971", "AE +971"),
    ("+966", "SA +966"),
    ("+880", "BD +880"),
    ("+92", "PK +92"),
];

#[component]
pub fn InputPhone(#[prop(into)] value: RwSignal<String>) -> impl IntoView {
    let code = RwSignal::new("+1".to_string());
    let number = RwSignal::new(String::new());

    // Sync combined value whenever code or number changes
    let update_combined = move || {
        let c = code.get();
        let n = number.get();
        value.set(if n.is_empty() {
            c
        } else {
            format!("{} {}", c, n)
        });
    };

    let on_number_input = move |e: leptos::ev::Event| {
        // Strip non-digit chars — light auto-format
        let raw = event_target_value(&e);
        let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        number.set(digits);
        update_combined();
    };

    // When code changes, re-sync
    let on_code_change = move || update_combined();

    // Wire code signal change
    Effect::new(move |_| {
        let _ = code.get(); // track
        on_code_change();
    });

    let tel_class = format!(
        "flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground shadow-elev-sm hover:border-ring/60 {} {}",
        CONTROL_MOTION, FOCUS_RING
    );

    view! {
        <div class="flex gap-2 w-full">
            <div class="w-36 shrink-0">
                <Select value=code placeholder="Code">
                    <SelectContent>
                        {COUNTRIES.iter().map(|(dial, label)| {
                            let dial = dial.to_string();
                            let label = label.to_string();
                            view! {
                                <SelectItem value=dial.clone()>{label}</SelectItem>
                            }
                        }).collect::<Vec<_>>()}
                    </SelectContent>
                </Select>
            </div>
            <input
                type="tel"
                placeholder="Phone number"
                class=tel_class
                prop:value=move || number.get()
                on:input=on_number_input
            />
        </div>
    }
}
