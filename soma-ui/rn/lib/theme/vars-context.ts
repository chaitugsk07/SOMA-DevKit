import { createContext, useContext } from 'react';
import { lightVars } from './tokens';

/**
 * The active CSS-variable token record provided by the nearest theme provider.
 * All keys match those in lightVars/darkVars/delightsLightVars; values are HSL
 * triplet strings ("H S% L%").
 *
 * RN Modals render outside the provider View, so Modal-based components
 * (Dialog, Sheet) consume this context to re-inject the correct palette.
 * Native colour props (ActivityIndicator, SVG stroke) use hslFromVar() to
 * convert a triplet into a concrete hsl() string.
 */
export type ThemeVarsRecord = { [K in keyof typeof lightVars]: string };

/** Falls back to Palantir lightVars if no provider is above. */
export const ThemeVarsContext = createContext<ThemeVarsRecord>(lightVars as ThemeVarsRecord);

export function useThemeVars(): ThemeVarsRecord {
  return useContext(ThemeVarsContext);
}

/**
 * Convert an HSL triplet string ("H S% L%") to a CSS hsl() colour string
 * suitable for native RN colour props (ActivityIndicator color, SVG stroke…).
 * Example: "100 32% 30%" → "hsl(100, 32%, 30%)"
 */
export function hslFromVar(triplet: string): string {
  const parts = triplet.trim().split(/\s+/);
  // parts: ["100", "32%", "30%"] — already has % on s and l
  return `hsl(${parts[0]}, ${parts[1]}, ${parts[2]})`;
}
