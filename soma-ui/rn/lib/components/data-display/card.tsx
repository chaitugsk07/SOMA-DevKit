import { View, Text, type ViewProps, type TextProps } from 'react-native';
import { cn } from '@/lib/utils/cn';

type DivProps = ViewProps & { className?: string };
type TxtProps = TextProps & { className?: string };

export function Card({ className, ...props }: DivProps) {
  return (
    <View
      className={cn('rounded-lg border border-border bg-card', className)}
      {...props}
    />
  );
}

export function CardHeader({ className, ...props }: DivProps) {
  return <View className={cn('gap-1.5 p-4', className)} {...props} />;
}

export function CardTitle({ className, ...props }: TxtProps) {
  return (
    <Text className={cn('font-heading-semibold text-lg text-card-foreground', className)} {...props} />
  );
}

export function CardDescription({ className, ...props }: TxtProps) {
  return (
    <Text className={cn('font-body text-sm text-muted-foreground', className)} {...props} />
  );
}

export function CardContent({ className, ...props }: DivProps) {
  return <View className={cn('p-4 pt-0', className)} {...props} />;
}

export function CardFooter({ className, ...props }: DivProps) {
  return <View className={cn('flex-row items-center p-4 pt-0', className)} {...props} />;
}
