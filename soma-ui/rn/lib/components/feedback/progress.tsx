import { View } from 'react-native';
import { cn } from '@/lib/utils/cn';

export type ProgressProps = {
  /** 0–100. */
  value: number;
  className?: string;
};

export function Progress({ value, className }: ProgressProps) {
  const pct = Math.max(0, Math.min(100, value));
  return (
    <View
      accessibilityRole="progressbar"
      accessibilityValue={{ min: 0, max: 100, now: pct }}
      className={cn('h-2 overflow-hidden rounded-full bg-muted', className)}
    >
      <View className="h-full rounded-full bg-primary" style={{ width: `${pct}%` }} />
    </View>
  );
}
