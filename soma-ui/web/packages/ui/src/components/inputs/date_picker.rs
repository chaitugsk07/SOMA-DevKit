use crate::components::shared::{CONTROL_MOTION, FOCUS_RING};
use crate::icons::{icondata, Icon};
use leptos::prelude::*;

// ponytail: single-month view, single-date selection, ISO "YYYY-MM-DD" value.
// Ceiling: no range selection, no min/max constraints, no keyboard nav in grid.
// Upgrade path: add min/max props + range RwSignal<(Option<String>, Option<String>)>,
// arrow-key roving focus over the day grid.

/// Returns (year, month 0-based, day 1-based) for today using js_sys::Date.
fn today() -> (i32, i32, i32) {
    let d = js_sys::Date::new_0();
    (
        d.get_full_year() as i32,
        d.get_month() as i32,
        d.get_date() as i32,
    )
}

/// Days in `month` (0-based) of `year`.
fn days_in_month(year: i32, month: i32) -> i32 {
    // js_sys::Date::new_with_year_month_day(y, next_month, 0) = last day of month
    let next = if month == 11 { 0 } else { month + 1 };
    let y = if month == 11 { year + 1 } else { year };
    js_sys::Date::new_with_year_month_day(y as u32, next as i32, 0).get_date() as i32
}

/// 0 = Sunday … 6 = Saturday for the first day of month.
fn weekday_of_first(year: i32, month: i32) -> i32 {
    js_sys::Date::new_with_year_month_day(year as u32, month as i32, 1).get_day() as i32
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const DAY_HEADERS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

#[component]
pub fn DatePicker(
    #[prop(into)] value: RwSignal<Option<String>>,
    #[prop(optional, into)] placeholder: String,
) -> impl IntoView {
    let open = RwSignal::new(false);

    let (ty, tm, _td) = today();
    let view_year = RwSignal::new(ty);
    let view_month = RwSignal::new(tm); // 0-based

    let placeholder = StoredValue::new(if placeholder.is_empty() {
        "Pick a date".to_string()
    } else {
        placeholder
    });

    let trigger_label = move || value.get().unwrap_or_else(|| placeholder.get_value());
    let has_value = move || value.get().is_some();

    let trigger_class = format!(
        "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 text-sm text-foreground shadow-elev-sm hover:border-ring/60 {} {}",
        CONTROL_MOTION, FOCUS_RING
    );

    view! {
        <div class="relative w-full">
            // Trigger button (input-styled)
            <button
                type="button"
                class=trigger_class
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <span class=move || if has_value() { "text-foreground" } else { "text-muted-foreground" }>
                    {trigger_label}
                </span>
                <Icon icon=Signal::derive(|| icondata::LuCalendar) width="16" height="16" />
            </button>

            // Calendar popover
            <Show when=move || open.get()>
                <div class="fixed inset-0 z-40" on:click=move |_| open.set(false) />
                <div class="absolute z-50 mt-2 w-72 rounded-md border border-border bg-card p-3 shadow-md animate-scale-in">
                    // Month navigation header
                    <div class="flex items-center justify-between mb-3">
                        <button
                            type="button"
                            class="flex h-7 w-7 items-center justify-center rounded hover:bg-accent hover:text-accent-foreground text-foreground"
                            on:click=move |_| {
                                let m = view_month.get();
                                if m == 0 {
                                    view_month.set(11);
                                    view_year.update(|y| *y -= 1);
                                } else {
                                    view_month.set(m - 1);
                                }
                            }
                        >
                            <Icon icon=Signal::derive(|| icondata::LuChevronLeft) width="14" height="14" />
                        </button>
                        <span class="text-sm font-medium text-foreground">
                            {move || format!("{} {}", MONTH_NAMES[view_month.get() as usize], view_year.get())}
                        </span>
                        <button
                            type="button"
                            class="flex h-7 w-7 items-center justify-center rounded hover:bg-accent hover:text-accent-foreground text-foreground"
                            on:click=move |_| {
                                let m = view_month.get();
                                if m == 11 {
                                    view_month.set(0);
                                    view_year.update(|y| *y += 1);
                                } else {
                                    view_month.set(m + 1);
                                }
                            }
                        >
                            <Icon icon=Signal::derive(|| icondata::LuChevronRight) width="14" height="14" />
                        </button>
                    </div>

                    // Weekday headers
                    <div class="grid grid-cols-7 mb-1">
                        {DAY_HEADERS.iter().map(|h| view! {
                            <div class="flex h-8 items-center justify-center text-xs text-muted-foreground">{*h}</div>
                        }).collect::<Vec<_>>()}
                    </div>

                    // Day grid
                    {move || {
                        let year = view_year.get();
                        let month = view_month.get();
                        let first_dow = weekday_of_first(year, month);
                        let days = days_in_month(year, month);
                        let (ty2, tm2, td2) = today();
                        let sel = value.get();

                        let mut cells: Vec<_> = Vec::new();
                        // Empty leading cells
                        for _ in 0..first_dow {
                            cells.push(view! { <div /> }.into_any());
                        }
                        for day in 1..=days {
                            let iso = format!("{:04}-{:02}-{:02}", year, month + 1, day);
                            let iso_clone = iso.clone();
                            let is_today = year == ty2 && month == tm2 && day == td2;
                            let is_selected = sel.as_deref() == Some(iso.as_str());
                            let base = "flex h-8 w-8 mx-auto items-center justify-center rounded text-sm cursor-pointer";
                            let style_class = if is_selected {
                                format!("{} bg-primary text-primary-foreground", base)
                            } else if is_today {
                                format!("{} text-primary font-semibold hover:bg-accent hover:text-accent-foreground", base)
                            } else {
                                format!("{} text-foreground hover:bg-accent hover:text-accent-foreground", base)
                            };
                            cells.push(view! {
                                <div
                                    class=style_class
                                    on:click=move |_| {
                                        value.set(Some(iso_clone.clone()));
                                        open.set(false);
                                    }
                                >
                                    {day}
                                </div>
                            }.into_any());
                        }
                        view! {
                            <div class="grid grid-cols-7 gap-y-1">
                                {cells}
                            </div>
                        }
                    }}
                </div>
            </Show>
        </div>
    }
}
