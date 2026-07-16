import { Children, type ReactNode } from 'react';
import { View, type ViewProps } from 'react-native';
import { FadeIn } from './fade-in';

export type AnimateGroupProps = ViewProps & {
  children: ReactNode;
  /** ms between each child's entrance. */
  stagger?: number;
  className?: string;
  /** Applied to each child's animated wrapper (e.g. 'flex-1' for grid items). */
  itemClassName?: string;
};

/** Fades children in one after another (staggered list/grid entrance). */
export function AnimateGroup({
  children,
  stagger = 60,
  className,
  itemClassName,
  ...props
}: AnimateGroupProps) {
  return (
    <View className={className} {...props}>
      {Children.map(children, (child, i) => (
        <FadeIn delay={i * stagger} className={itemClassName}>
          {child}
        </FadeIn>
      ))}
    </View>
  );
}
