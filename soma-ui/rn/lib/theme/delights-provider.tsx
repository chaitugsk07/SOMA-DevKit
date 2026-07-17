import { type ReactNode } from 'react';
import { View } from 'react-native';
import { vars } from 'nativewind';
import { delightsLightVars } from './tokens';
import { ThemeVarsContext, type ThemeVarsRecord } from './vars-context';

/**
 * Wraps s-delights screens in the warm-cream brand tokens (light-only).
 * Use instead of SomaThemeProvider for s-delights surfaces. Other soma services
 * keep the base SomaThemeProvider — the two coexist.
 *
 * Also provides ThemeVarsContext so that Modal-based components (Dialog, Sheet)
 * and native-colour components (Spinner, AnimatedRing) re-inject the correct
 * Delights palette rather than falling back to the base light theme.
 */
export function SomaDelightsThemeProvider({ children }: { children: ReactNode }) {
  return (
    <ThemeVarsContext.Provider value={delightsLightVars as ThemeVarsRecord}>
      <View style={vars(delightsLightVars)} className="flex-1 bg-background">
        {children}
      </View>
    </ThemeVarsContext.Provider>
  );
}
