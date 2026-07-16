import { View } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { CountUp } from '@/lib/components/motion/count-up';
import { cn } from '@/lib/utils/cn';

export type StatCardProps = {
  label: string;
  value: number;
  prefix?: string;
  suffix?: string;
  decimals?: number;
  /** Percent change vs previous period; positive = up (green), negative = down (red). */
  trend?: number;
  className?: string;
};

/** Admin dashboard metric: animated value + trend delta. */
export function StatCard({ label, value, prefix, suffix, decimals = 0, trend, className }: StatCardProps) {
  const up = trend != null && trend >= 0;
  return (
    <View className={cn('gap-1.5 rounded-xl border border-border bg-card p-4', className)}>
      <Text variant="muted" className="text-xs uppercase tracking-wide">{label}</Text>
      <CountUp to={value} prefix={prefix} suffix={suffix} decimals={decimals} variant="h1" />
      {trend != null && (
        <View className="flex-row items-center gap-1">
          <Text className={cn('text-xs font-body-semibold', up ? 'text-success' : 'text-destructive')}>
            {up ? '▲' : '▼'} {Math.abs(trend).toFixed(1)}%
          </Text>
          <Text variant="muted" className="text-xs">vs last week</Text>
        </View>
      )}
    </View>
  );
}
