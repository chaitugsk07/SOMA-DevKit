import { Text as RNText, type TextProps as RNTextProps } from 'react-native';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils/cn';

// Headings use Rajdhani; body/muted/small use Outfit — mirrors web/flutter.
const textVariants = cva('text-foreground', {
  variants: {
    variant: {
      h1: 'font-heading-bold text-3xl',
      h2: 'font-heading-semibold text-2xl',
      h3: 'font-heading-semibold text-xl',
      p: 'font-body text-base',
      muted: 'font-body text-sm text-muted-foreground',
      small: 'font-body text-xs',
    },
  },
  defaultVariants: { variant: 'p' },
});

export type TextProps = RNTextProps &
  VariantProps<typeof textVariants> & { className?: string };

export function Text({ variant, className, ...props }: TextProps) {
  return <RNText className={cn(textVariants({ variant }), className)} {...props} />;
}

export { textVariants };
