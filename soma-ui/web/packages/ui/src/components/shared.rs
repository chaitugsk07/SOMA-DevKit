/// Shared constants and tiny helpers used across multiple components.
/// All items are `pub(crate)` — not part of the public API.

// ─── Elevation / motion primitives ───────────────────────────────────────────

/// Accessible focus ring using `focus-visible` (keyboard-only, no mouse halo).
/// Composes onto any interactive element. Uses `--ring` and `--background` tokens.
pub(crate) const FOCUS_RING: &str =
    "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background";

/// Inset variant of `FOCUS_RING` — no offset, ring drawn inside the element.
/// Used by accordion, tabs, and toggle_group where an outset halo would overflow.
pub(crate) const FOCUS_RING_INSET: &str =
    "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset";

/// Smooth 150 ms ease-out transition for all properties (shadow, color, transform).
/// Apply to any component that animates on hover/focus/active.
pub(crate) const CONTROL_MOTION: &str = "transition-all duration-150 ease-out";

/// Subtle press-down affordance on activation. Combine with `CONTROL_MOTION`
/// so the scale animates rather than snapping.
pub(crate) const PRESSABLE: &str = "active:scale-[0.97]";

// ─────────────────────────────────────────────────────────────────────────────

/// Common menu-item class shared by dropdown_menu, context_menu, menubar, combobox,
/// select, multi_select, and command.
pub(crate) const MENU_ITEM_CLASS: &str =
    "flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm cursor-pointer transition-colors hover:bg-accent hover:text-accent-foreground text-foreground";

/// Menu separator div class, repeated across dropdown_menu, context_menu, and menubar.
pub(crate) const MENU_SEPARATOR_CLASS: &str = "-mx-1 my-1 h-px bg-border";

/// Returns the disabled styling classes when `disabled` is true, otherwise an empty string.
pub(crate) fn disabled_cls(disabled: bool) -> &'static str {
    if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        ""
    }
}
