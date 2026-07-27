// Tokens ported exactly from soma-ui web/theme/tokens.css.
// HSL triplets ("H S% L%") consumed by NativeWind vars() and resolved
// through hsl(var(--x)) in tailwind.config.js.

export const lightVars = {
  '--background': '0 0% 100%',
  '--foreground': '240 10% 4%',
  '--card': '0 0% 100%',
  '--card-foreground': '240 10% 4%',
  '--muted': '240 5% 96%',
  '--muted-foreground': '240 4% 46%',
  '--primary': '240 6% 10%',
  '--primary-foreground': '0 0% 98%',
  '--secondary': '240 5% 96%',
  '--secondary-foreground': '240 6% 10%',
  '--accent': '240 5% 96%',
  '--accent-foreground': '240 6% 10%',
  '--destructive': '6 66% 44%',
  '--destructive-foreground': '0 0% 100%',
  '--success': '160 60% 29%',
  '--success-foreground': '0 0% 100%',
  '--warning': '30 80% 32%',
  '--warning-foreground': '0 0% 100%',
  '--info': '214 74% 44%',
  '--info-foreground': '0 0% 100%',
  '--contrast': '240 6% 10%',
  '--contrast-foreground': '0 0% 100%',
  '--border': '240 6% 90%',
  '--input': '240 6% 90%',
  '--ring': '240 5% 65%',
} as const;

export const darkVars = {
  '--background': '240 10% 4%',
  '--foreground': '0 0% 98%',
  '--card': '240 6% 7%',
  '--card-foreground': '0 0% 98%',
  '--muted': '240 4% 16%',
  '--muted-foreground': '240 5% 65%',
  '--primary': '0 0% 98%',
  '--primary-foreground': '240 6% 10%',
  '--secondary': '240 4% 16%',
  '--secondary-foreground': '0 0% 98%',
  '--accent': '240 4% 16%',
  '--accent-foreground': '0 0% 98%',
  '--destructive': '6 75% 68%',
  '--destructive-foreground': '240 6% 10%',
  '--success': '160 52% 58%',
  '--success-foreground': '240 6% 10%',
  '--warning': '38 85% 62%',
  '--warning-foreground': '240 6% 10%',
  '--info': '214 80% 70%',
  '--info-foreground': '240 6% 10%',
  '--contrast': '0 0% 98%',
  '--contrast-foreground': '240 6% 10%',
  '--border': '240 4% 16%',
  '--input': '240 4% 16%',
  '--ring': '240 5% 45%',
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
export const delightsLightVars = {
  '--background': '30 33% 97%',        // warm unbleached paper (#FAF8F5)
  '--foreground': '0 0% 18%',          // brand charcoal (#2D2D2D)
  '--card': '0 0% 100%',
  '--card-foreground': '0 0% 18%',
  '--muted': '39 47% 89%',             // soft sand (#F0E6D4)
  '--muted-foreground': '0 0% 38%',
  '--primary': '0 0% 18%',             // charcoal keeps CTA contrast accessible
  '--primary-foreground': '0 0% 100%',
  '--secondary': '39 47% 89%',
  '--secondary-foreground': '0 0% 18%',
  '--accent': '125 20% 90%',
  '--accent-foreground': '125 25% 31%', // leaf green (#4A7C4F)
  '--destructive': '7 57% 49%',
  '--destructive-foreground': '0 0% 100%',
  '--success': '125 25% 31%',
  '--success-foreground': '0 0% 100%',
  '--warning': '36 66% 32%',           // dark amber, warnings only
  '--warning-foreground': '0 0% 100%',
  '--info': '210 45% 36%',             // restrained slate blue, information only
  '--info-foreground': '0 0% 100%',
  '--contrast': '20 6% 10%',
  '--contrast-foreground': '0 0% 100%',
  '--border': '34 22% 84%',
  '--input': '30 33% 97%',
  '--ring': '40 88% 44%',               // turmeric gold (#D4930D)
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
