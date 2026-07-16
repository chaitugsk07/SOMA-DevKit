import { ActivityIndicator } from 'react-native';
import { useThemeVars, hslFromVar } from '@/lib/theme/vars-context';

export type SpinnerProps = {
  size?: 'small' | 'large';
  /**
   * Indicator color. ActivityIndicator's `color` is a native prop — it takes a
   * concrete colour string, not a CSS var(). When omitted, defaults to the
   * active theme's --primary token so it themes correctly under both
   * SomaThemeProvider (Palantir blue) and SomaDelightsThemeProvider (near-black).
   */
  color?: string;
};

export function Spinner({ size = 'small', color }: SpinnerProps) {
  const activeVars = useThemeVars();
  const resolvedColor = color ?? hslFromVar(activeVars['--primary']);
  return <ActivityIndicator size={size} color={resolvedColor} accessibilityLabel="Loading" />;
}
