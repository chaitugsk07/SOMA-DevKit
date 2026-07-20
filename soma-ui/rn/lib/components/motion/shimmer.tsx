import { View } from 'react-native';
import { MotiView } from 'moti';
import { useReducedMotion } from '@/lib/hooks';
import { cn } from '@/lib/utils/cn';

export type ShimmerProps = {
  className?: string;
};

/** Looping opacity pulse for loading placeholders. */
export function Shimmer({ className }: ShimmerProps) {
  const reducedMotion = useReducedMotion();

  if (reducedMotion) {
    return <View className={cn('rounded-md bg-muted', className)} />;
  }

  return (
    <MotiView
      from={{ opacity: 0.5 }}
      animate={{ opacity: 1 }}
      transition={{ type: 'timing', duration: 800, loop: true }}
      className={cn('rounded-md bg-muted', className)}
    />
  );
}
