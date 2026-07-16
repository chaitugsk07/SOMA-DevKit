import { View } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type OrderStep = { label: string; time?: string };

export type OrderStatusProps = {
  steps: OrderStep[];
  /** Index of the current step (0-based). Steps before it are complete. */
  current: number;
  className?: string;
};

/** Vertical order-progress timeline with an animated connector + pulsing current dot. */
export function OrderStatus({ steps, current, className }: OrderStatusProps) {
  return (
    <View className={className}>
      {steps.map((step, i) => {
        const done = i < current;
        const active = i === current;
        const last = i === steps.length - 1;
        return (
          <View key={step.label} className="flex-row gap-3">
            {/* dot + connector column */}
            <View className="items-center">
              <View
                className={cn(
                  'h-5 w-5 items-center justify-center rounded-full border-2',
                  done || active ? 'border-primary bg-primary' : 'border-border bg-card',
                )}
              >
                {done && <Text className="text-[10px] leading-none text-primary-foreground">✓</Text>}
                {active && (
                  <MotiView
                    from={{ scale: 0.6, opacity: 0.6 }}
                    animate={{ scale: 1, opacity: 1 }}
                    transition={{ type: 'timing', duration: 700, loop: true }}
                    className="h-2 w-2 rounded-full bg-primary-foreground"
                  />
                )}
              </View>
              {!last && (
                <View className="w-0.5 flex-1 overflow-hidden bg-border">
                  <MotiView
                    from={{ translateY: -40 }}
                    animate={{ translateY: done ? 0 : -40 }}
                    transition={{ type: 'timing', duration: 400, delay: i * 120 }}
                    className="h-full w-full bg-primary"
                  />
                </View>
              )}
            </View>
            {/* label */}
            <View className={cn('flex-1 pb-6', last && 'pb-0')}>
              <Text
                className={cn(
                  'font-body-medium text-sm',
                  done || active ? 'text-foreground' : 'text-muted-foreground',
                )}
              >
                {step.label}
              </Text>
              {step.time && <Text variant="small" className="text-muted-foreground">{step.time}</Text>}
            </View>
          </View>
        );
      })}
    </View>
  );
}
