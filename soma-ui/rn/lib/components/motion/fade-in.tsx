import { type ReactNode } from 'react';
import { MotiView } from 'moti';

export type FadeInProps = {
  children: ReactNode;
  /** ms before the animation starts (for staggering). */
  delay?: number;
  /** ms the fade takes. */
  duration?: number;
  className?: string;
};

/** Fades + lifts content in on mount. */
export function FadeIn({ children, delay = 0, duration = 300, className }: FadeInProps) {
  return (
    <MotiView
      from={{ opacity: 0, translateY: 8 }}
      animate={{ opacity: 1, translateY: 0 }}
      transition={{ type: 'timing', duration, delay }}
      className={className}
    >
      {children}
    </MotiView>
  );
}
