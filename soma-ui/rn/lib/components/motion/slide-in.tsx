import { type ReactNode } from 'react';
import { View } from 'react-native';
import { MotiView } from 'moti';
import { useReducedMotion } from '@/lib/hooks';

type Direction = 'left' | 'right' | 'up' | 'down';

const offset: Record<Direction, { translateX?: number; translateY?: number }> = {
  left: { translateX: -24 },
  right: { translateX: 24 },
  up: { translateY: -24 },
  down: { translateY: 24 },
};

export type SlideInProps = {
  children: ReactNode;
  from?: Direction;
  delay?: number;
  duration?: number;
  className?: string;
};

/** Slides + fades content in from an edge on mount. */
export function SlideIn({
  children,
  from = 'down',
  delay = 0,
  duration = 350,
  className,
}: SlideInProps) {
  const reducedMotion = useReducedMotion();

  if (reducedMotion) {
    return <View className={className}>{children}</View>;
  }

  return (
    <MotiView
      from={{ opacity: 0, ...offset[from] }}
      animate={{ opacity: 1, translateX: 0, translateY: 0 }}
      transition={{ type: 'timing', duration, delay }}
      className={className}
    >
      {children}
    </MotiView>
  );
}
