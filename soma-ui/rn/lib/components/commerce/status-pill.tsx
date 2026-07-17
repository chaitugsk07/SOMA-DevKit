import { View } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type StatusTone = 'neutral' | 'active' | 'success' | 'warning';

export type StatusPillProps = {
  label: string;
  tone?: StatusTone;
  /** Show a pulsing live dot (for 'active'). */
  live?: boolean;
  className?: string;
};

const tones: Record<StatusTone, { dot: string; text: string; bg: string }> = {
  neutral: { dot: 'bg-muted-foreground', text: 'text-muted-foreground', bg: 'bg-muted' },
  active: { dot: 'bg-primary', text: 'text-primary', bg: 'bg-primary/10' },
  success: { dot: 'bg-success', text: 'text-success', bg: 'bg-success/10' },
  warning: { dot: 'bg-warning', text: 'text-warning', bg: 'bg-warning/10' },
};

/** Status label with a colored (optionally pulsing) dot. */
export function StatusPill({ label, tone = 'neutral', live = false, className }: StatusPillProps) {
  const t = tones[tone];
  return (
    <View className={cn('flex-row items-center gap-1.5 self-start rounded-full px-2.5 py-1', t.bg, className)}>
      {live ? (
        <MotiView
          from={{ opacity: 0.4, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ type: 'timing', duration: 700, loop: true }}
          className={cn('h-2 w-2 rounded-full', t.dot)}
        />
      ) : (
        <View className={cn('h-2 w-2 rounded-full', t.dot)} />
      )}
      <Text className={cn('font-body-medium text-xs', t.text)}>{label}</Text>
    </View>
  );
}
