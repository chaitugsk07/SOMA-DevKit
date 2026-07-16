import { View } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { CountUp } from '@/lib/components/motion/count-up';
import { cn } from '@/lib/utils/cn';

export type EarningsStatProps = {
  label: string;
  value: number;
  prefix?: string;
  suffix?: string;
  decimals?: number;
  className?: string;
};

/** Stat tile with an animated (count-up) value — for driver/earnings dashboards. */
export function EarningsStat({ label, value, prefix, suffix, decimals = 0, className }: EarningsStatProps) {
  return (
    <View className={cn('gap-1 rounded-xl border border-border bg-card p-4', className)}>
      <CountUp to={value} prefix={prefix} suffix={suffix} decimals={decimals} variant="h2" />
      <Text variant="muted" className="text-xs">{label}</Text>
    </View>
  );
}
