import { View } from 'react-native';
import { Text } from '@/lib/components/data-display/text';

export type StepDotsProps = { current: number; total: number };

/** Numeric progress with a quiet hairline track. */
export function StepDots({ current, total }: StepDotsProps) {
  return (
    <View
      className="flex-row items-center gap-4"
      accessibilityRole="progressbar"
      accessibilityLabel={`Step ${current} of ${total}`}
    >
      <Text className="font-body-medium text-[10px] tracking-[2px] text-muted-foreground">
        {String(current).padStart(2, '0')} / {String(total).padStart(2, '0')}
      </Text>
      <View className="h-px flex-1 bg-border">
        <View
          className="h-px bg-primary"
          style={{ width: `${Math.min(100, Math.max(0, current / total * 100))}%` }}
        />
      </View>
    </View>
  );
}
