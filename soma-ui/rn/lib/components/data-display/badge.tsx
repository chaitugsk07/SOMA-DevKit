import { View, Text } from 'react-native';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils/cn';

const badgeVariants = cva('self-start rounded-md px-2 py-0.5', {
  variants: {
    variant: {
      default: 'bg-primary',
      secondary: 'bg-secondary',
      destructive: 'bg-destructive',
      success: 'bg-success',
      outline: 'border border-border bg-transparent',
    },
  },
  defaultVariants: { variant: 'default' },
});

const badgeTextVariants = cva('font-body-medium text-xs', {
  variants: {
    variant: {
      default: 'text-primary-foreground',
      secondary: 'text-secondary-foreground',
      destructive: 'text-destructive-foreground',
      success: 'text-success-foreground',
      outline: 'text-foreground',
    },
  },
  defaultVariants: { variant: 'default' },
});

export type BadgeProps = VariantProps<typeof badgeVariants> & {
  label: string;
  className?: string;
};

export function Badge({ variant, label, className }: BadgeProps) {
  return (
    <View className={cn(badgeVariants({ variant }), className)}>
      <Text className={cn(badgeTextVariants({ variant }))}>{label}</Text>
    </View>
  );
}

export { badgeVariants };
