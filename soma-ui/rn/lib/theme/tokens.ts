// Palantir tokens ported EXACTLY from soma-ui web/theme/tokens.css.
// HSL triplets ("H S% L%") consumed by NativeWind vars() and resolved
// through hsl(var(--x)) in tailwind.config.js.

export const lightVars = {
  '--background': '216 33% 97%',
  '--foreground': '213 24% 14%',
  '--card': '0 0% 100%',
  '--card-foreground': '213 24% 14%',
  '--muted': '216 24% 93%',
  '--muted-foreground': '215 16% 42%',
  '--primary': '214 80% 47%',
  '--primary-foreground': '0 0% 100%',
  '--secondary': '216 24% 93%',
  '--secondary-foreground': '213 24% 18%',
  '--accent': '216 24% 90%',
  '--accent-foreground': '213 24% 18%',
  '--destructive': '358 66% 48%',
  '--destructive-foreground': '0 0% 100%',
  '--success': '149 52% 37%',
  '--success-foreground': '0 0% 100%',
  '--border': '214 20% 85%',
  '--input': '214 20% 85%',
  '--ring': '214 80% 50%',
} as const;

export const darkVars = {
  '--background': '213 18% 11%',
  '--foreground': '215 20% 95%',
  '--card': '215 16% 15%',
  '--card-foreground': '215 20% 95%',
  '--muted': '216 14% 20%',
  '--muted-foreground': '215 14% 64%',
  '--primary': '214 82% 56%',
  '--primary-foreground': '0 0% 100%',
  '--secondary': '216 14% 20%',
  '--secondary-foreground': '215 20% 95%',
  '--accent': '216 14% 24%',
  '--accent-foreground': '215 20% 95%',
  '--destructive': '358 62% 56%',
  '--destructive-foreground': '0 0% 100%',
  '--success': '149 47% 44%',
  '--success-foreground': '0 0% 100%',
  '--border': '216 13% 24%',
  '--input': '216 13% 24%',
  '--ring': '214 90% 62%',
} as const;

// ─── s-delights brand tokens ────────────────────────────────────────────────
// "Quiet luxury meets radical transparency" — warm cream stage, white cards,
// near-black ink primary (NOT blue), product photos are the only color.
// Light-only (dark mode is out of scope for the s-delights launch).
//
// This deliberately EVOLVES the print design-system's greyscale-only semantics
// for app UX: functional destructive/success color is required (e.g. cancel
// subscription needs a warning affordance; streaks/delivery status need a
// positive signal), and the signature pressed-leaf green accent surfaces
// selected/active states. Near-black ink CTAs and product-color restraint
// (productAccents only for SKU identity dots) are PRESERVED.
//
// WCAG contrast ratios (verified 2026-07-02):
//   muted-foreground on background : 5.27:1  ✓ AA
//   muted-foreground on card       : 5.74:1  ✓ AA
//   foreground on background       : 13.00:1 ✓ AAA
//   primary-foreground on primary  : 17.52:1 ✓ AAA
//   white on destructive           : 6.66:1  ✓ AA
//   white on success               : 6.65:1  ✓ AA
//   accent-foreground on accent    : 7.64:1  ✓ AA
export const delightsLightVars = {
  '--background': '32 22% 96%',        // warm cream, slightly warmer
  '--foreground': '20 7% 17%',         // richer ink
  '--card': '0 0% 100%',
  '--card-foreground': '20 6% 10%',
  '--muted': '30 12% 89%',
  '--muted-foreground': '24 6% 40%',   // WCAG fix: was 59% (~2.9:1 on cream)
  '--primary': '20 6% 10%',            // near-black ink CTA — brand signature, KEEP
  '--primary-foreground': '0 0% 100%',
  '--secondary': '30 14% 92%',
  '--secondary-foreground': '20 6% 10%',
  '--accent': '92 24% 89%',            // soft pressed-leaf tint — selected/active surfaces
  '--accent-foreground': '100 38% 22%', // deep leaf green on accent
  '--destructive': '8 52% 40%',        // warm terracotta, destructive actions only
  '--destructive-foreground': '0 0% 100%',
  '--success': '100 32% 30%',          // pressed-leaf green (signature accent)
  '--success-foreground': '0 0% 100%',
  '--border': '30 10% 87%',
  '--input': '30 14% 93%',
  '--ring': '100 32% 30%',             // green focus ring
} as const;

// Product accent colors — RESTRICTED: only for SKU/product-identity dots, never
// buttons/backgrounds/text/borders/icons. Referenced directly (not theme vars).
export const productAccents = {
  vitalityGreen: '#6B8F3C', // Green Morning
  beetRuby: '#8B2252', // Beetroot Recharge
  turmericGold: '#C4972A', // Turmeric Ginger Shot
  citrusAmber: '#D4882B', // Citrus Immunity
  berryDeep: '#5C2D50',
  earthBrown: '#6B5344',
} as const;

export type ProductAccent = keyof typeof productAccents;
