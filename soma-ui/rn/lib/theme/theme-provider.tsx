import { createContext, useContext, useState, type ReactNode } from 'react';
import { View } from 'react-native';
import { vars } from 'nativewind';
import { darkVars, lightVars } from './tokens';
import { ThemeVarsContext, type ThemeVarsRecord } from './vars-context';

type ColorScheme = 'light' | 'dark';

type ThemeContextValue = {
  scheme: ColorScheme;
  toggle: () => void;
};

export const ThemeContext = createContext<ThemeContextValue | null>(null);

export function useTheme(): ThemeContextValue {
  const ctx = useContext(ThemeContext);
  if (!ctx) throw new Error('useTheme must be used within <SomaThemeProvider>');
  return ctx;
}

export function SomaThemeProvider({
  children,
  initial = 'light',
}: {
  children: ReactNode;
  initial?: ColorScheme;
}) {
  const [scheme, setScheme] = useState<ColorScheme>(initial);
  const toggle = () => setScheme((s) => (s === 'light' ? 'dark' : 'light'));

  const activeVars = scheme === 'light' ? lightVars : darkVars;

  return (
    <ThemeContext.Provider value={{ scheme, toggle }}>
      <ThemeVarsContext.Provider value={activeVars as ThemeVarsRecord}>
        <View style={vars(activeVars)} className="flex-1 bg-background">
          {children}
        </View>
      </ThemeVarsContext.Provider>
    </ThemeContext.Provider>
  );
}
