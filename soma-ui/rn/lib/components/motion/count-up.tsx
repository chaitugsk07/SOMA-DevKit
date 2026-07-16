import { useEffect, useState } from 'react';
import { useSharedValue, useAnimatedReaction, withTiming, Easing } from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';
import { Text, type TextProps } from '@/lib/components/data-display/text';

export type CountUpProps = Omit<TextProps, 'children'> & {
  to: number;
  /** ms to reach the target. */
  duration?: number;
  /** Decimal places to show. */
  decimals?: number;
  prefix?: string;
  suffix?: string;
};

/** Counts a number up from 0 to `to` on mount — for stats/dashboards. */
export function CountUp({ to, duration = 1000, decimals = 0, prefix = '', suffix = '', ...textProps }: CountUpProps) {
  const progress = useSharedValue(0);
  const [display, setDisplay] = useState('0');

  useEffect(() => {
    progress.value = 0;
    progress.value = withTiming(to, { duration, easing: Easing.out(Easing.cubic) });
  }, [to, duration, progress]);

  useAnimatedReaction(
    () => progress.value,
    (v) => scheduleOnRN(setDisplay, v.toFixed(decimals)),
    [decimals],
  );

  return <Text {...textProps}>{`${prefix}${display}${suffix}`}</Text>;
}
