import { View, type ViewProps } from 'react-native';
import { MotiView } from 'moti';
import { cn } from '@/lib/utils/cn';

export type SkeletonProps = ViewProps & {
  className?: string;
  /** Pass false for a static placeholder (no pulse). */
  animated?: boolean;
};

/** Loading placeholder with a subtle pulse. */
export function Skeleton({ className, animated = true, ...props }: SkeletonProps) {
  if (!animated) {
    return <View className={cn('rounded-md bg-muted', className)} {...props} />;
  }
  return (
    <MotiView
      from={{ opacity: 0.5 }}
      animate={{ opacity: 1 }}
      transition={{ type: 'timing', duration: 800, loop: true }}
      className={cn('rounded-md bg-muted', className)}
      {...props}
    />
  );
}

// ---------------------------------------------------------------------------
// Content-shaped presets — composed from the base Skeleton above.
// ---------------------------------------------------------------------------

export type SkeletonTextProps = {
  /** Number of text lines to render. Defaults to 3. */
  lines?: number;
  className?: string;
};

/**
 * Multi-line text placeholder. Line widths vary naturally.
 *
 * Usage:
 *   <SkeletonText lines={2} />
 */
export function SkeletonText({ lines = 3, className }: SkeletonTextProps) {
  // Widths cycle through to give a natural ragged-right look.
  const widths = ['w-full', 'w-4/5', 'w-3/5', 'w-full', 'w-2/3'];
  return (
    <View className={cn('gap-2', className)}>
      {Array.from({ length: lines }).map((_, i) => (
        <Skeleton key={i} className={cn('h-3', widths[i % widths.length])} />
      ))}
    </View>
  );
}

export type SkeletonCardProps = {
  className?: string;
};

/**
 * Card-shaped placeholder — image block on top, two text lines below.
 *
 * Usage:
 *   <SkeletonCard />
 */
export function SkeletonCard({ className }: SkeletonCardProps) {
  return (
    <View className={cn('overflow-hidden rounded-xl bg-card', className)}>
      {/* Image / hero block */}
      <Skeleton className="h-40 w-full rounded-none" />
      <View className="gap-2 p-3">
        <Skeleton className="h-4 w-3/4" />
        <Skeleton className="h-3 w-1/2" />
      </View>
    </View>
  );
}

export type SkeletonRowProps = {
  className?: string;
};

/**
 * Row-shaped placeholder — avatar circle on the left, two lines on the right.
 *
 * Usage:
 *   <SkeletonRow />
 */
export function SkeletonRow({ className }: SkeletonRowProps) {
  return (
    <View className={cn('flex-row items-center gap-3', className)}>
      {/* Avatar circle */}
      <Skeleton className="h-10 w-10 rounded-full" />
      <View className="flex-1 gap-2">
        <Skeleton className="h-3.5 w-2/3" />
        <Skeleton className="h-3 w-1/2" />
      </View>
    </View>
  );
}
