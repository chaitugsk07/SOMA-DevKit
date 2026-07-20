import { Pressable, Text, View } from 'react-native';
import { cn } from '@/lib/utils/cn';

export type SegmentedControlOption<T extends string = string> = { label: string; value: T };

export type SegmentedControlProps<T extends string = string> = {
  options: SegmentedControlOption<T>[];
  value: T | null;
  onChange: (value: T) => void;
  className?: string;
  disabled?: boolean;
  accessibilityLabel?: string;
};

export function SegmentedControl<T extends string = string>({
  options,
  value,
  onChange,
  className,
  disabled,
  accessibilityLabel,
}: SegmentedControlProps<T>) {
  return (
    <View
      accessibilityLabel={accessibilityLabel}
      className={cn('flex-row flex-wrap gap-2', className)}
    >
      {options.map((option) => {
        const selected = option.value === value;
        return (
          <Pressable
            key={option.value}
            accessibilityRole="button"
            accessibilityState={{ selected, disabled }}
            accessibilityLabel={option.label}
            disabled={disabled}
            onPress={() => onChange(option.value)}
            className={cn(
              'min-h-11 justify-center rounded-md border px-3 py-2',
              selected
                ? 'border-primary bg-primary'
                : 'border-border bg-muted',
              disabled && 'opacity-50',
            )}
          >
            <Text
              className={cn(
                'font-body-medium text-sm',
                selected ? 'text-primary-foreground' : 'text-foreground',
              )}
            >
              {option.label}
            </Text>
          </Pressable>
        );
      })}
    </View>
  );
}
