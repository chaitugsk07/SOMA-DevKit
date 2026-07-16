import { type ReactNode } from 'react';
import { MotiView } from 'moti';

type Direction = 'up' | 'down' | 'left' | 'right' | 'none';

const from: Record<Direction, { translateX?: number; translateY?: number }> = {
  up: { translateY: 32 },
  down: { translateY: -32 },
  left: { translateX: 32 },
  right: { translateX: -32 },
  none: {},
};

export type RevealProps = {
  children: ReactNode;
  direction?: Direction;
  delay?: number;
  duration?: number;
  className?: string;
};

// ponytail: reveals on mount, not on scroll-into-view (RN has no IntersectionObserver).
// For most sections that mount when navigated-to this is identical; if you need true
// scroll-triggered reveal, gate `animate` on an onLayout/scroll-position check.
export function Reveal({
  children,
  direction = 'up',
  delay = 0,
  duration = 500,
  className,
}: RevealProps) {
  return (
    <MotiView
      from={{ opacity: 0, ...from[direction] }}
      animate={{ opacity: 1, translateX: 0, translateY: 0 }}
      transition={{ type: 'timing', duration, delay }}
      className={className}
    >
      {children}
    </MotiView>
  );
}
