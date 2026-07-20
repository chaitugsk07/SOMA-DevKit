import { View, type ViewProps } from 'react-native';
import { responsiveValue, useBreakpoint } from '@/lib/hooks';
import { cn } from '@/lib/utils/cn';

export type ScreenContainerProps = ViewProps & {
  className?: string;
  maxWidth?: number;
};

export function ScreenContainer({
  className,
  maxWidth = 1120,
  style,
  ...props
}: ScreenContainerProps) {
  const breakpoint = useBreakpoint();
  const paddingHorizontal = responsiveValue(breakpoint, {
    phone: 20,
    tablet: 32,
    desktop: 48,
  });

  return (
    <View
      className={cn('w-full self-center', className)}
      style={[{ maxWidth, paddingHorizontal }, style]}
      {...props}
    />
  );
}
