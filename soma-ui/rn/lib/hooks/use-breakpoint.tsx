import { createContext, useContext, useState, type ReactNode } from 'react';
import { View, useWindowDimensions, type LayoutChangeEvent } from 'react-native';

/** Tailwind/NativeWind default breakpoints (md=768, lg=1024). */
export const BREAKPOINTS = { md: 768, lg: 1024 } as const;

export type Breakpoint = {
  width: number;
  isPhone: boolean; // < 768
  isTablet: boolean; // 768–1023
  isDesktop: boolean; // >= 1024
};

function classify(width: number): Breakpoint {
  return {
    width,
    isPhone: width < BREAKPOINTS.md,
    isTablet: width >= BREAKPOINTS.md && width < BREAKPOINTS.lg,
    isDesktop: width >= BREAKPOINTS.lg,
  };
}

// null = no provider above → fall back to the real browser window.
const WidthContext = createContext<number | null>(null);

/**
 * Reactive breakpoint for THIS layout context.
 *
 * On web, NativeWind `md:`/`lg:` classes and react-native's `useWindowDimensions()`
 * both key off the browser window — they can't see a narrower parent container. So
 * inside a device frame (PhoneFrame) they'd wrongly report the full window width.
 * This hook prefers a `BreakpointProvider`'s measured container width when present,
 * and only falls back to the real window when a screen renders standalone/full-bleed.
 *
 * Author responsive layout through this hook (or <Columns>) so it stays correct
 * inside a frame; `md:` class prefixes remain window-only.
 */
export function useBreakpoint(): Breakpoint {
  const container = useContext(WidthContext);
  const { width: windowWidth } = useWindowDimensions();
  return classify(container ?? windowWidth);
}

/**
 * Measures its own width and exposes it to descendant useBreakpoint()/<Columns>
 * calls, so a fixed-width frame drives layout instead of the browser window.
 */
export function BreakpointProvider({ children }: { children: ReactNode }) {
  const [width, setWidth] = useState<number | null>(null);
  const onLayout = (e: LayoutChangeEvent) => {
    const w = Math.round(e.nativeEvent.layout.width);
    if (w > 0 && w !== width) setWidth(w);
  };
  return (
    <View className="flex-1" onLayout={onLayout}>
      {/* Don't publish width until measured, so the first paint isn't misclassified. */}
      {width === null ? children : <WidthContext.Provider value={width}>{children}</WidthContext.Provider>}
    </View>
  );
}

/** Pick a value by breakpoint, falling back downward: desktop→tablet→phone. */
export function responsiveValue<T>(bp: Breakpoint, values: { phone: T; tablet?: T; desktop?: T }): T {
  if (bp.isDesktop && values.desktop !== undefined) return values.desktop;
  if (!bp.isPhone && values.tablet !== undefined) return values.tablet;
  return values.phone;
}
