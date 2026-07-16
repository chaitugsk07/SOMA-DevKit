import { Children, type ReactNode } from 'react';
import { View, type DimensionValue } from 'react-native';
import { cn } from '@/lib/utils/cn';
import { useBreakpoint } from './use-breakpoint';

/**
 * Render children in a responsive grid. Column counts can't be expressed with
 * NativeWind class prefixes (they'd need a static class per count), so this
 * computes each item's width from the live breakpoint.
 *
 * Gutter is applied as inner padding (half-gap per side) on a negative-margin
 * row, so each cell can take an EXACT `100/cols %` basis and always fit `cols`
 * per line — avoids the "basis + parent gap overflows and wraps to 1" trap
 * (React Native has no CSS `calc()` to subtract the gap from a percentage).
 *
 * <Columns phone={1} tablet={2} desktop={3} gap={16}>...</Columns>
 */
export function Columns({
  children,
  phone = 1,
  tablet,
  desktop,
  gap = 16,
  className,
}: {
  children: ReactNode;
  phone?: number;
  tablet?: number;
  desktop?: number;
  gap?: number;
  className?: string;
}) {
  const bp = useBreakpoint();
  const cols = bp.isDesktop ? (desktop ?? tablet ?? phone) : bp.isTablet ? (tablet ?? phone) : phone;
  const items = Children.toArray(children);
  const half = gap / 2;
  const basis = `${100 / cols}%` as DimensionValue;

  return (
    <View className={cn('flex-row flex-wrap', className)} style={{ marginHorizontal: -half }}>
      {items.map((child, i) => (
        <View key={i} style={{ flexBasis: basis, maxWidth: basis, paddingHorizontal: half, marginBottom: gap }}>
          {child}
        </View>
      ))}
    </View>
  );
}
