import { Pressable, View } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';
import { useReducedMotion } from '@/lib/hooks';

export type QuantityStepperProps = {
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  className?: string;
};

function StepButton({ label, onPress, disabled }: { label: string; onPress: () => void; disabled?: boolean }) {
  return (
    <Pressable
      onPress={onPress}
      disabled={disabled}
      accessibilityRole="button"
      accessibilityLabel={label === '−' ? 'Decrease quantity' : 'Increase quantity'}
      className={cn('h-11 w-11 items-center justify-center active:bg-accent', disabled && 'opacity-40')}
    >
      <Text className="font-heading-semibold text-lg text-foreground">{label}</Text>
    </Pressable>
  );
}

/** −/count/+ control with an animated count. */
export function QuantityStepper({ value, onChange, min = 1, max = 99, className }: QuantityStepperProps) {
  const reducedMotion = useReducedMotion();

  return (
    <View className={cn('flex-row items-center self-start overflow-hidden rounded-lg border border-border', className)}>
      <StepButton label="−" onPress={() => onChange(Math.max(min, value - 1))} disabled={value <= min} />
      <View className="h-11 w-11 items-center justify-center border-x border-border">
        {reducedMotion ? (
          <Text className="font-body-semibold text-sm text-foreground">{value}</Text>
        ) : (
          <MotiView key={value} from={{ opacity: 0, translateY: -6 }} animate={{ opacity: 1, translateY: 0 }} transition={{ type: 'timing', duration: 150 }}>
            <Text className="font-body-semibold text-sm text-foreground">{value}</Text>
          </MotiView>
        )}
      </View>
      <StepButton label="+" onPress={() => onChange(Math.min(max, value + 1))} disabled={value >= max} />
    </View>
  );
}
