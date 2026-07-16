use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use leptos::prelude::*;
use leptos::web_sys;

// ponytail: focus approach — Vec<NodeRef<html::Input>> (one per box).
// On input: update combined value, call .focus() on next NodeRef.
// On Backspace in empty box: .focus() on previous NodeRef.
// Ceiling: numeric-only (inputmode=numeric). Full tel/alpha OTP is the upgrade path
// (remove inputmode, allow any char in the filter closure).

#[component]
pub fn InputOtp(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = 6)] length: usize,
) -> impl IntoView {
    // One NodeRef per box
    let refs: Vec<NodeRef<leptos::html::Input>> = (0..length).map(|_| NodeRef::new()).collect();
    let refs = StoredValue::new(refs);

    // Derive chars from the combined value (pad/truncate to `length`)
    let char_at = move |i: usize| {
        let v = value.get();
        let ch: Vec<char> = v.chars().collect();
        ch.get(i).copied()
    };

    let focus_box = move |i: usize| {
        refs.with_value(|r| {
            if let Some(nr) = r.get(i) {
                if let Some(el) = nr.get() {
                    let _ = el.focus();
                }
            }
        });
    };

    let box_class = format!(
        "h-10 w-10 text-center rounded-md border border-input bg-background text-sm text-foreground shadow-elev-sm hover:border-ring/60 {} {}",
        CONTROL_MOTION, FOCUS_RING
    );

    view! {
        <div class="flex gap-2">
            {(0..length).map(|i| {
                let box_class = box_class.clone();
                let on_input = move |e: leptos::ev::Event| {
                    let raw = event_target_value(&e);
                    // Keep only last digit typed (maxlength=1, but we sanitise)
                    let ch = raw.chars().last().filter(|c| c.is_ascii_digit());
                    value.update(|v| {
                        let mut chars: Vec<char> = v.chars().collect();
                        // Ensure vec is long enough
                        while chars.len() <= i { chars.push('\0'); }
                        if let Some(c) = ch {
                            chars[i] = c;
                        } else {
                            chars[i] = '\0';
                        }
                        // Rebuild trimming trailing nulls
                        let s: String = chars.iter()
                            .take(length)
                            .map(|c| if *c == '\0' { ' ' } else { *c })
                            .collect();
                        *v = s.trim_end().to_string();
                    });
                    if ch.is_some() && i + 1 < length {
                        focus_box(i + 1);
                    }
                };

                let on_keydown = move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Backspace" {
                        let current_char = char_at(i);
                        // If current box already empty, move back and clear previous
                        if (current_char.is_none() || current_char == Some(' ')) && i > 0 {
                            value.update(|v| {
                                let mut chars: Vec<char> = v.chars().collect();
                                if i - 1 < chars.len() {
                                    chars[i - 1] = '\0';
                                    let s: String = chars.iter()
                                        .take(length)
                                        .map(|c| if *c == '\0' { ' ' } else { *c })
                                        .collect();
                                    *v = s.trim_end().to_string();
                                }
                            });
                            focus_box(i - 1);
                        }
                    }
                };

                let nr = refs.with_value(|r| r[i]);

                view! {
                    <input
                        node_ref=nr
                        type="text"
                        inputmode="numeric"
                        maxlength="1"
                        class=box_class
                        prop:value=move || {
                            char_at(i).filter(|c| *c != ' ').map(|c| c.to_string()).unwrap_or_default()
                        }
                        on:input=on_input
                        on:keydown=on_keydown
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
