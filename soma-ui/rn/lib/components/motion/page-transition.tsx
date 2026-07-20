import { type ReactNode } from 'react';
import { View } from 'react-native';
import { MotiView } from 'moti';
import { useReducedMotion } from '@/lib/hooks';

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
  const reducedMotion = useReducedMotion();
  const v = enter[variant];

  if (reducedMotion) {
    return (
      <View key={transitionKey} className={className} style={{ flex: 1 }}>
        {children}
      </View>
    );
  }

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
