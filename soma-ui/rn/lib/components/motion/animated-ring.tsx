import { useEffect } from 'react';
import { View } from 'react-native';
import Svg, { Circle } from 'react-native-svg';
import Animated, { useSharedValue, useAnimatedProps, withTiming, Easing } from 'react-native-reanimated';
import { useThemeVars, hslFromVar } from '@/lib/theme/vars-context';
import { useReducedMotion } from '@/lib/hooks';
import { CountUp } from './count-up';

const AnimatedCircle = Animated.createAnimatedComponent(Circle);

export type AnimatedRingProps = {
  /** 0–100. */
  value: number;
  size?: number;
  strokeWidth?: number;
  duration?: number;
  /** Show the % in the center. */
  showLabel?: boolean;
  /**
   * Filled arc colour. Defaults to the active theme's --success token
   * (pressed-leaf green under Delights, emerald under Palantir).
   * Pass a concrete hsl/hex/rgb string to override.
   */
  color?: string;
  /**
   * Track (unfilled arc) colour. Defaults to the active theme's --border token.
   * Pass a concrete hsl/hex/rgb string to override.
   */
  trackColor?: string;
};

/** Circular progress ring that fills on mount — for scores/stats. */
export function AnimatedRing({
  value,
  size = 96,
  strokeWidth = 8,
  duration = 1000,
  showLabel = true,
  color,
  trackColor,
}: AnimatedRingProps) {
  const reducedMotion = useReducedMotion();
  const activeVars = useThemeVars();
  const resolvedColor = color ?? hslFromVar(activeVars['--success']);
  const resolvedTrack = trackColor ?? hslFromVar(activeVars['--border']);

  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const progress = useSharedValue(0);

  useEffect(() => {
    progress.value = 0;
    progress.value = withTiming(Math.max(0, Math.min(100, value)) / 100, {
      duration: reducedMotion ? 0 : duration,
      easing: Easing.out(Easing.cubic),
    });
  }, [value, duration, progress, reducedMotion]);

  const animatedProps = useAnimatedProps(() => ({
    strokeDashoffset: circumference * (1 - progress.value),
  }));

  return (
    <View style={{ width: size, height: size }} className="items-center justify-center">
      <Svg width={size} height={size} style={{ position: 'absolute', transform: [{ rotate: '-90deg' }] }}>
        <Circle cx={size / 2} cy={size / 2} r={radius} stroke={resolvedTrack} strokeWidth={strokeWidth} fill="none" />
        <AnimatedCircle
          cx={size / 2}
          cy={size / 2}
          r={radius}
          stroke={resolvedColor}
          strokeWidth={strokeWidth}
          fill="none"
          strokeLinecap="round"
          strokeDasharray={circumference}
          animatedProps={animatedProps}
        />
      </Svg>
      {showLabel && (
        <CountUp
          to={value}
          duration={reducedMotion ? 0 : duration}
          suffix="%"
          variant="h3"
        />
      )}
    </View>
  );
}
