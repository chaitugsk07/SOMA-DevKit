import { type ReactNode } from 'react';
import { View } from 'react-native';
import { MotiView } from 'moti';
import { useReducedMotion } from '@/lib/hooks';

export type PulseProps = {
  children: ReactNode;
  /** Largest scale in the pulse. */
  scale?: number;
  /** ms per pulse half-cycle. */
  duration?: number;
  className?: string;
};

/** Gentle looping scale pulse to draw attention (badges, CTAs, live dots). */
export function Pulse({ children, scale = 1.08, duration = 700, className }: PulseProps) {
  const reducedMotion = useReducedMotion();

  if (reducedMotion) {
    return <View className={className}>{children}</View>;
  }

  return (
    <MotiView
      from={{ scale: 1 }}
      animate={{ scale }}
      transition={{ type: 'timing', duration, loop: true }}
      className={className}
    >
      {children}
    </MotiView>
  );
}
