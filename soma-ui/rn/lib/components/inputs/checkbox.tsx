import { Pressable, Text } from 'react-native';
import { cn } from '@/lib/utils/cn';

export type CheckboxProps = {
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  disabled?: boolean;
  className?: string;
};

export function Checkbox({ checked, onCheckedChange, disabled, className }: CheckboxProps) {
  return (
    <Pressable
      accessibilityRole="checkbox"
      accessibilityState={{ checked, disabled }}
      disabled={disabled}
      onPress={() => onCheckedChange(!checked)}
      className={cn(
        'h-5 w-5 items-center justify-center rounded-sm border',
        checked ? 'border-primary bg-primary' : 'border-input bg-transparent',
        disabled && 'opacity-50',
        className,
      )}
    >
      {checked && (
        <Text className="text-xs leading-none text-primary-foreground">✓</Text>
      )}
    </Pressable>
  );
}
