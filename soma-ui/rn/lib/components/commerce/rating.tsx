import { View } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type RatingProps = {
  /** 0–5, fractional allowed (rounded to nearest half for display). */
  value: number;
  /** Number of reviews, shown in parentheses. */
  count?: number;
  size?: 'sm' | 'md';
  className?: string;
};

/** Star rating with optional review count. */
export function Rating({ value, count, size = 'sm', className }: RatingProps) {
  const rounded = Math.round(value * 2) / 2;
  const star = size === 'sm' ? 'text-xs' : 'text-base';
  return (
    <View className={cn('flex-row items-center gap-1', className)}>
      <View className="flex-row">
        {[1, 2, 3, 4, 5].map((i) => {
          const fill = rounded >= i ? 'full' : rounded >= i - 0.5 ? 'half' : 'empty';
          return (
            <Text key={i} className={cn(star, fill === 'empty' ? 'text-border' : 'text-primary')}>
              {fill === 'half' ? '⯨' : '★'}
            </Text>
          );
        })}
      </View>
      {count != null && (
        <Text className={cn('font-body text-muted-foreground', size === 'sm' ? 'text-xs' : 'text-sm')}>
          ({count.toLocaleString()})
        </Text>
      )}
    </View>
  );
}
