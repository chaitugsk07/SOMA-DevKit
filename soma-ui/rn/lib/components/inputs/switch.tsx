import { Pressable, View } from 'react-native';
import { cn } from '@/lib/utils/cn';

export type SwitchProps = {
  value: boolean;
  onValueChange: (value: boolean) => void;
  disabled?: boolean;
  className?: string;
};

export function Switch({ value, onValueChange, disabled, className }: SwitchProps) {
  return (
    <Pressable
      accessibilityRole="switch"
      accessibilityState={{ checked: value, disabled }}
      disabled={disabled}
      onPress={() => onValueChange(!value)}
      className={cn(
        'h-6 w-11 justify-center rounded-full px-0.5',
        value ? 'bg-primary' : 'bg-input',
        disabled && 'opacity-50',
        className,
      )}
    >
      <View
        className={cn(
          'h-5 w-5 rounded-full bg-card',
          value ? 'self-end' : 'self-start',
        )}
      />
    </Pressable>
  );
}
