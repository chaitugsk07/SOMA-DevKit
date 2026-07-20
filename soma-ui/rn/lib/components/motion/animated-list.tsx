import { type ReactNode } from 'react';
import { View, type ViewProps } from 'react-native';
import Animated, { FadeIn, FadeOut, LinearTransition } from 'react-native-reanimated';
import { useReducedMotion } from '@/lib/hooks';

export type AnimatedListItemProps = ViewProps & {
  children: ReactNode;
  className?: string;
};

/**
 * One list row that animates in on mount, out on removal, and smoothly
 * reflows when siblings change. Give each a stable `key`.
 */
export function AnimatedListItem({ children, className, ...props }: AnimatedListItemProps) {
  const reducedMotion = useReducedMotion();

  if (reducedMotion) {
    return (
      <View className={className} {...props}>
        {children}
      </View>
    );
  }

  return (
    <Animated.View
      entering={FadeIn.duration(250)}
      exiting={FadeOut.duration(200)}
      layout={LinearTransition.springify().damping(18)}
      className={className}
      {...props}
    >
      {children}
    </Animated.View>
  );
}

export type AnimatedListProps = ViewProps & {
  children: ReactNode;
  className?: string;
};

/** Container for AnimatedListItem rows (add/remove/reorder animate). */
export function AnimatedList({ children, className, ...props }: AnimatedListProps) {
  return (
    <View className={className} {...props}>
      {children}
    </View>
  );
}
