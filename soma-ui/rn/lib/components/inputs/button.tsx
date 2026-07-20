import React from 'react';
import { ActivityIndicator, Pressable, Text, View, type PressableProps } from 'react-native';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils/cn';
import { useThemeVars, hslFromVar } from '@/lib/theme/vars-context';

const buttonVariants = cva(
  'flex-row items-center justify-center rounded-lg',
  {
    variants: {
      variant: {
        default: 'bg-primary active:opacity-90',
        secondary: 'bg-secondary active:opacity-90',
        destructive: 'bg-destructive active:opacity-90',
        outline: 'border border-border bg-transparent active:bg-accent',
        ghost: 'bg-transparent active:bg-accent',
        link: 'bg-transparent',
      },
      size: {
        sm: 'h-11 px-3',
        default: 'h-11 px-5',
        lg: 'h-12 px-7',
        icon: 'h-11 w-11',
      },
    },
    defaultVariants: { variant: 'default', size: 'default' },
  },
);

const labelVariants = cva('font-body-medium text-sm', {
  variants: {
    variant: {
      default: 'text-primary-foreground',
      secondary: 'text-secondary-foreground',
      destructive: 'text-destructive-foreground',
      outline: 'text-foreground',
      ghost: 'text-foreground',
      link: 'text-primary underline',
    },
  },
  defaultVariants: { variant: 'default' },
});


export type ButtonProps = Omit<PressableProps, 'children'> &
  VariantProps<typeof buttonVariants> & {
    label?: string;
    loading?: boolean;
    className?: string;
    // Narrowed from PressableProps: Button never called function children, so we make that explicit.
    children?: React.ReactNode;
  };

export function Button({
  variant,
  size,
  label,
  loading,
  className,
  disabled,
  children,
  ...props
}: ButtonProps) {
  const isLoading = loading ?? false;
  const vars = useThemeVars();
  const indicatorColor =
    variant === 'default' ? hslFromVar(vars['--primary-foreground']) :
    variant === 'destructive' ? hslFromVar(vars['--destructive-foreground']) :
    hslFromVar(vars['--foreground']);

  return (
    <Pressable
      accessibilityRole="button"
      disabled={disabled || isLoading}
      className={cn(buttonVariants({ variant, size }), disabled && 'opacity-50', className)}
      {...props}
    >
      {isLoading && (
        <ActivityIndicator size="small" color={indicatorColor} style={{ position: 'absolute' }} />
      )}
      {/* Keep layout stable during loading: hide content but preserve button width */}
      <View style={{ flexDirection: 'row', alignItems: 'center', opacity: isLoading ? 0 : 1 }}>
        {label != null ? (
          <Text className={cn(labelVariants({ variant }))}>{label}</Text>
        ) : (
          children
        )}
      </View>
    </Pressable>
  );
}

export { buttonVariants };
