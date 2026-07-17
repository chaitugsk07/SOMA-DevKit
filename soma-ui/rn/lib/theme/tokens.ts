// Tokens ported exactly from soma-ui web/theme/tokens.css.
// HSL triplets ("H S% L%") consumed by NativeWind vars() and resolved
// through hsl(var(--x)) in tailwind.config.js.

export const lightVars = {
  '--background': '30 38% 96%',
  '--foreground': '19 24% 13%',
  '--card': '33 43% 98%',
  '--card-foreground': '19 24% 13%',
  '--muted': '33 30% 92%',
  '--muted-foreground': '24 12% 43%',
  '--primary': '0 17% 36%',
  '--primary-foreground': '30 28% 95%',
  '--secondary': '34 28% 90%',
  '--secondary-foreground': '19 24% 18%',
  '--accent': '30 32% 89%',
  '--accent-foreground': '19 24% 18%',
  '--destructive': '6 66% 44%',
  '--destructive-foreground': '0 0% 100%',
  '--success': '160 60% 29%',
  '--success-foreground': '0 0% 100%',
  '--warning': '30 80% 32%',
  '--warning-foreground': '0 0% 100%',
  '--info': '214 74% 44%',
  '--info-foreground': '0 0% 100%',
  '--contrast': '34 15% 10%',
  '--contrast-foreground': '0 0% 100%',
  '--border': '34 26% 86%',
  '--input': '34 22% 85%',
  '--ring': '0 32% 46%',
} as const;

export const darkVars = {
  '--background': '22 24% 7%',
  '--foreground': '33 30% 93%',
  '--card': '24 18% 13%',
  '--card-foreground': '33 30% 93%',
  '--muted': '24 16% 22%',
  '--muted-foreground': '28 12% 63%',
  '--primary': '9 33% 43%',
  '--primary-foreground': '30 25% 95%',
  '--secondary': '24 14% 19%',
  '--secondary-foreground': '33 30% 93%',
  '--accent': '24 16% 22%',
  '--accent-foreground': '33 30% 93%',
  '--destructive': '6 75% 68%',
  '--destructive-foreground': '22 24% 10%',
  '--success': '160 52% 58%',
  '--success-foreground': '22 24% 10%',
  '--warning': '38 85% 62%',
  '--warning-foreground': '22 24% 10%',
  '--info': '214 80% 70%',
  '--info-foreground': '22 24% 10%',
  '--contrast': '33 30% 93%',
  '--contrast-foreground': '22 24% 10%',
  '--border': '26 15% 26%',
  '--input': '26 16% 31%',
  '--ring': '9 52% 60%',
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
  '--warning': '36 66% 32%',           // dark amber, warnings only
  '--warning-foreground': '0 0% 100%',
  '--info': '210 45% 36%',             // restrained slate blue, information only
  '--info-foreground': '0 0% 100%',
  '--contrast': '20 6% 10%',
  '--contrast-foreground': '0 0% 100%',
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
