import { View, Text } from 'react-native';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils/cn';

const alertVariants = cva('rounded-lg border border-l-4 p-4 shadow-sm', {
  variants: {
    variant: {
      default: 'border-border bg-card',
      destructive: 'border-destructive bg-destructive/10',
      success: 'border-success bg-success/10',
      warning: 'border-warning bg-warning/10',
      info: 'border-info bg-info/10',
    },
  },
  defaultVariants: { variant: 'default' },
});

const titleColor = {
  default: 'text-foreground',
  destructive: 'text-destructive',
  success: 'text-success',
  warning: 'text-warning',
  info: 'text-info',
} as const;

export type AlertProps = VariantProps<typeof alertVariants> & {
  title: string;
  description?: string;
  className?: string;
};

export function Alert({ variant, title, description, className }: AlertProps) {
  const v = variant ?? 'default';
  return (
    <View className={cn(alertVariants({ variant }), className)}>
      <Text className={cn('font-heading-semibold text-base', titleColor[v])}>{title}</Text>
      {description && (
        <Text className="mt-1 font-body text-sm text-muted-foreground">{description}</Text>
      )}
    </View>
  );
}

export { alertVariants };
