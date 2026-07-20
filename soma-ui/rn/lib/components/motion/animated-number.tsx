import { useEffect, useState } from 'react';
import { useSharedValue, useAnimatedReaction, withTiming } from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';
import { Text, type TextProps } from '@/lib/components/data-display/text';
import { useReducedMotion } from '@/lib/hooks';

export type AnimatedNumberProps = Omit<TextProps, 'children'> & {
  value: number;
  duration?: number;
  decimals?: number;
};

/** Smoothly tweens between values whenever `value` changes (live counters/badges). */
export function AnimatedNumber({ value, duration = 400, decimals = 0, ...textProps }: AnimatedNumberProps) {
  const reducedMotion = useReducedMotion();
  const current = useSharedValue(value);
  const [display, setDisplay] = useState(value.toFixed(decimals));

  useEffect(() => {
    current.value = withTiming(value, { duration: reducedMotion ? 0 : duration });
  }, [value, duration, current, reducedMotion]);

  useAnimatedReaction(
    () => current.value,
    (v) => scheduleOnRN(setDisplay, v.toFixed(decimals)),
    [decimals],
  );

  return <Text {...textProps}>{display}</Text>;
}
