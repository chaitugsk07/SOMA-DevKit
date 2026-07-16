import { type ReactNode } from 'react';
import { MotiView } from 'moti';

export type PageTransitionVariant = 'fade' | 'slide' | 'scale';

export type PageTransitionProps = {
  children: ReactNode;
  /**
   * Change this whenever the page/screen changes (e.g. the route name). The
   * wrapper remounts on a new key, replaying the enter animation.
   */
  transitionKey: string | number;
  variant?: PageTransitionVariant;
  duration?: number;
  className?: string;
};

const enter: Record<PageTransitionVariant, { from: object; animate: object }> = {
  fade: {
    from: { opacity: 0 },
    animate: { opacity: 1 },
  },
  slide: {
    from: { opacity: 0, translateX: 24 },
    animate: { opacity: 1, translateX: 0 },
  },
  scale: {
    from: { opacity: 0, scale: 0.97 },
    animate: { opacity: 1, scale: 1 },
  },
};

/** Animates page/screen content in on each `transitionKey` change. */
export function PageTransition({
  children,
  transitionKey,
  variant = 'fade',
  duration = 280,
  className,
}: PageTransitionProps) {
  const v = enter[variant];
  return (
    <MotiView
      key={transitionKey}
      from={v.from}
      animate={v.animate}
      transition={{ type: 'timing', duration }}
      className={className}
      style={{ flex: 1 }}
    >
      {children}
    </MotiView>
  );
}
